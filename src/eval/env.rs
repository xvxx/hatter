use {
    crate::{builtin, compile, Args, ErrorKind, Fn, OMap, Result, Stmt, Symbol, Tag, Value},
    std::{
        cell::{Ref, RefCell},
        collections::HashMap,
        mem,
        rc::Rc,
    },
};

/// Eval a list of Stmts and return the last's Value.
pub fn eval(src: &str) -> Result<Value> {
    let mut env = Env::new();
    let val = env.eval_src(src);
    debug_assert_eq!(env.scopes.len(), 1);
    val
}

/// Render source to a String.
pub fn render(source: &str) -> Result<String> {
    let mut env = Env::new();
    let string = env.render(source);
    debug_assert_eq!(env.scopes.len(), 1);
    string
}

/// Error-ish that lets us abort what we're doing.
#[derive(Debug, PartialEq)]
pub enum Jump {
    Break,
    Continue,
    Return(Value),
}

/// You know what it is.
pub type Scope = Rc<RefCell<HashMap<String, Value>>>;

/// Env is the top-level environment in which a Hatter template is
/// evaluated and rendered.
#[derive(Debug)]
pub struct Env {
    scopes: Vec<Scope>,
    out: String,
}

impl Default for Env {
    fn default() -> Env {
        let mut scope = HashMap::new();
        for (name, fun) in builtin::natives() {
            scope.insert(name, Value::Fn(Fn::Native(fun)));
        }
        for (name, fun) in builtin::specials() {
            scope.insert(name, Value::Fn(Fn::Special(fun)));
        }
        Env {
            scopes: vec![rcell!(scope)],
            out: String::new(),
        }
    }
}

impl Env {
    /// New, top-level Env.
    pub fn new() -> Env {
        Env::default()
    }

    /// Return and clear output.
    pub fn out(&mut self) -> String {
        mem::replace(&mut self.out, String::new())
    }

    /// Create an empty Args struct, to call a helper function.
    pub fn empty_args(&mut self) -> Args {
        Args::new(self, vec![])
    }

    /// Nearest (currently active) scope.
    pub fn scope(&self) -> &Scope {
        &self.scopes[self.scopes.len() - 1]
    }

    /// Add a new scope to the stack, which becomes active.
    pub fn push_scope(&mut self) {
        self.scopes.push(rcell!(HashMap::new()));
    }

    /// Kill the active scope.
    pub fn pop_scope(&mut self) {
        let _ = self.scopes.pop();
    }

    /// Nearest (currently active) scope.
    pub fn mut_scope(&mut self) -> &mut Scope {
        let len = self.scopes.len();
        &mut self.scopes[len - 1]
    }

    /// Stack of scopes.
    #[allow(unused)]
    pub(crate) fn scopes(&self) -> &[Scope] {
        &self.scopes
    }

    /// Does a value exist in this or any parent scopes?
    pub fn var_exists(&self, key: &str) -> bool {
        self.lookup(key).is_some()
    }

    /// Find a value, looking first in the most recently pushed scope.
    pub fn lookup(&self, key: &str) -> Option<Ref<'_, Value>> {
        self.find_scope(key)
            .map(|scope| Ref::map(scope.borrow(), |v| v.get(key).unwrap()))
    }

    /// Find the `Scope` in which a var exists, if there is one.
    fn find_scope(&self, key: &str) -> Option<&Scope> {
        for scope in self.scopes.iter().rev() {
            if scope.borrow().contains_key(key) {
                return Some(scope);
            }
        }
        None
    }

    /// Set a value to the nearest scope.
    pub fn set<V: Into<Value>>(&self, key: &str, val: V) {
        self.scope()
            .borrow_mut()
            .insert(key.to_string(), val.into());
    }

    /// Set a value in a parent scope, or create it in the nearest.
    pub fn update<V: Into<Value>>(&self, key: &str, val: V) {
        if let Some(scope) = self.find_scope(key) {
            scope
        } else {
            self.scope()
        }
        .borrow_mut()
        .insert(key.to_string(), val.into());
    }

    /// Add something to our render buffer without a trailing line.
    pub fn print<V: Into<Value>>(&mut self, val: V) {
        self.out.push_str(&val.into().to_string());
    }

    /// Render statements into a String.
    pub fn render(&mut self, source: &str) -> Result<String> {
        let stmts = compile(source)?;
        let autohtml = self.first_is_head(&stmts);
        if autohtml {
            self.print("<!DOCTYPE html>\n<html>");
        }
        self.printed_block(&stmts)?;
        if autohtml {
            self.print("\n</html>\n");
        }
        Ok(self.out())
    }

    /// Evaluate a block of statements, returning the last one.
    pub fn block(&mut self, stmts: &[Stmt]) -> Result<Value> {
        let mut out = Value::None;
        for stmt in stmts {
            let is_tag = matches!(stmt, Stmt::Tag(..));
            out = self.eval(&stmt)?;
            if is_tag {
                self.print(&out);
            }
        }
        Ok(out)
    }

    /// Evaluate a block of statements and print what each returns.
    /// "Outside of a tag, tags are printed.
    ///  Inside of a tag, everything is printed."
    pub fn printed_block(&mut self, stmts: &[Stmt]) -> Result<Value> {
        let mut out = Value::None;
        for stmt in stmts {
            if let Stmt::Word(w) = &stmt {
                if !self.is_keyword_or_var(w) {
                    self.print(w);
                    continue;
                }
            }
            out = self.eval(&stmt)?;
            if !matches!(out, Value::None) {
                self.print(&out);
            }
        }
        Ok(out)
    }

    /// Evaluate source code, returning its Value.
    pub fn eval_src(&mut self, src: &str) -> Result<Value> {
        compile(src).and_then(|c| self.block(&c))
    }

    /// Evaluate one statement, returning its Value.
    pub fn eval(&mut self, stmt: &Stmt) -> Result<Value> {
        Ok(match stmt {
            Stmt::None => Value::None,
            Stmt::Value(v) => v.clone(),
            Stmt::Tag(x) => self.eval_tag(x)?,
            Stmt::List(x) => x
                .iter()
                .map(|v| self.eval(&v))
                .collect::<Result<Vec<_>>>()?
                .into(),
            Stmt::Map(pairs) => {
                let mut map = OMap::new();
                for (k, v) in pairs {
                    map.insert(k, self.eval(v)?);
                }
                Value::Map(map.into())
            }
            Stmt::Word(word) => {
                match word.as_ref() {
                    "break" => return jump!(Jump::Break),
                    "continue" => return jump!(Jump::Continue),
                    _ => {}
                }
                if let Some(val) = self.lookup(&word) {
                    val.clone()
                } else {
                    return error!("can't find var: {}", word);
                }
            }
            Stmt::Call(target, args) => {
                // eval the target and see if it's a Hatter function
                if let Value::Fn(inner_fn) = self.eval(&target)? {
                    match inner_fn {
                        Fn::Special(f) => f.clone()(self, args)?,
                        Fn::Native(f) => {
                            let f = f.clone();
                            let args = args
                                .iter()
                                .map(|a| self.eval(&a))
                                .collect::<Result<Vec<_>>>()?;
                            f(Args::new(self, args))?
                        }
                        Fn::Fn(params, body, scope) => {
                            let mut kw_args = None;
                            if args.len() == 1 {
                                if let Stmt::Args(inner) = &args[0] {
                                    let mut args = vec![];
                                    for (i, name) in params.iter().enumerate() {
                                        if let Some(pair) = inner.iter().find(|(kw, _)| kw == name)
                                        {
                                            args.push((i, pair.1.clone()));
                                        }
                                    }
                                    kw_args = Some(args);
                                }
                            }
                            let kw_args = if let Some(mut inner) = kw_args {
                                inner.sort_by(|a, b| a.0.cmp(&b.0));
                                inner.into_iter().map(|p| p.1).collect::<Vec<_>>()
                            } else {
                                vec![]
                            };
                            let args = if !kw_args.is_empty() { &kw_args } else { args };

                            if params.len() != args.len() {
                                return error!(
                                    "expected {} args, got {}",
                                    params.len(),
                                    args.len()
                                );
                            }
                            self.scopes.push(scope);
                            self.push_scope();
                            for (i, a) in args.iter().enumerate() {
                                if let Some(name) = params.get(i) {
                                    let val = self.eval(&a)?;
                                    self.set(name, val);
                                }
                            }
                            let out = self.block(&body);
                            self.pop_scope();
                            self.pop_scope();
                            match out {
                                Ok(v) => v,
                                Err(e) => match e.kind {
                                    ErrorKind::Jump(Jump::Return(v)) => v,
                                    _ => return Err(e),
                                },
                            }
                        }
                    }
                } else {
                    return error!("can't find fn: {}", target.to_string());
                }
            }
            Stmt::Return(expr) => return jump!(Jump::Return(self.eval(expr)?)),
            Stmt::If(conds) => {
                for (test, body) in conds {
                    if self.eval(test)?.to_bool() {
                        self.push_scope();
                        self.block(body).map_err(|e| {
                            self.pop_scope();
                            e
                        })?;
                        self.pop_scope();
                        break;
                    }
                }
                Value::None
            }
            Stmt::For(..) => self.eval_for(stmt)?,
            Stmt::While(test, body) => {
                self.push_scope();
                while self.eval(&test)?.to_bool() {
                    match self.block(&body) {
                        Ok(_) => {}
                        Err(e) => match e.kind {
                            ErrorKind::Jump(Jump::Break) => break,
                            ErrorKind::Jump(Jump::Continue) => continue,
                            _ => {
                                self.pop_scope();
                                return Err(e)
                            },
                        },
                    }
                    self.scope().borrow_mut().clear();
                }
                self.pop_scope();
                Value::None
            }
            Stmt::Assign(name, expr, is_reassign) => {
                let exists = self.var_exists(name);
                if exists && !is_reassign {
                    return error!("{} already set", name);
                } else if !exists && *is_reassign {
                    return error!("{} is not set", name);
                }

                let val = self.eval(expr)?;
                if *is_reassign {
                    if self.lookup(name).unwrap().typename() != val.typename() {
                        return error!("{} is type {}", name, val.typename());
                    }
                    self.update(name, val);
                } else {
                    self.set(name, val);
                }
                Value::None
            }
            Stmt::Fn(params, body) => {
                Value::Fn(Fn::Fn(params.clone(), body.clone(), self.scope().clone()))
            }
            Stmt::Args(..) => unimplemented!(),
        })
    }

    /// Turn a <tag> into a Value::String.
    fn eval_tag(&mut self, tag: &Tag) -> Result<Value> {
        let mut out = String::new();
        out.push('<');

        // tag name
        let tagname = self.eval(&tag.tag)?.to_string();
        out.push_str(&tagname);
        out.push(' ');

        // don't run <debug> code in --release
        #[cfg(not(debug_assertions))]
        if tagname == "debug" {
            return Ok(Value::None);
        }

        // id
        if tag.id.is_some() {
            let val = self.eval(&tag.id)?;
            if val.to_bool() {
                out.push_str(&format!("id='{}'", val.to_string()));
                out.push(' ');
            }
        }

        // classes
        if !tag.classes.is_empty() {
            let mut classes = vec![];
            for class in &tag.classes {
                let val = self.eval(class)?;
                if val.to_bool() {
                    classes.push(val.to_string());
                }
            }
            out.push_str(&format!("class='{}'", classes.join(" ")));
            out.push(' ');
        }

        // attributes
        let is_form = tagname == "form";
        let is_link = tagname == "a";
        let mut has_href = !is_link;
        for (name, val) in &tag.attrs {
            let val = self.eval(val)?;
            if !val.to_bool() {
                continue;
            }

            let attr_name = self.eval(name)?.to_string();
            if is_form && matches!(attr_name.as_ref(), "GET" | "POST") {
                out.push_str(&format!("method='{}' action='{}'", attr_name, val));
            } else {
                if attr_name == "href" {
                    has_href = true;
                }
                match val {
                    Value::String(s) => out.push_str(&format!("{}='{}'", attr_name, s)),
                    Value::Bool(..) => out.push_str(&attr_name),
                    _ => out.push_str(&format!("{}={}", attr_name, val)),
                }
                out.push(' ');
            }
        }

        if is_link && !has_href {
            out.push_str(&"href='#' ");
        }

        // check for self-closing tag
        if tag.closed {
            out.push('/');
            out.push('>');
            out.push('\n');
            return Ok(out.into());
        }

        // trim trailing space
        if matches!(out.bytes().last(), Some(b' ')) {
            out.truncate(out.len() - 1);
        }

        // close tag
        out.push('>');

        // body
        let old_out = self.out();
        self.printed_block(&tag.body)?;
        out.push_str(&mem::replace(&mut self.out, old_out));

        // closing tag
        out.push_str(&format!("</{}>\n", tagname));

        // <debug> gets eval'd but not print'd in --debug mode
        if tagname == "debug" {
            Ok(Value::None)
        } else {
            Ok(out.into())
        }
    }

    /// Evaluate a for loop.
    fn eval_for(&mut self, stmt: &Stmt) -> Result<Value> {
        if let Stmt::For(key, val, expr, body) = stmt {
            match self.eval(&expr)? {
                Value::List(list) => {
                    self.inner_for(key, val, list.borrow().iter().enumerate(), body)?
                }
                Value::Map(map) => self.inner_for(key, val, map.borrow().iter(), body)?,
                v => return error!("expected List or Map, got {:?}", v),
            }
        }

        Ok(Value::None)
    }

    /// Shared "inner" for loop, over both maps and lists.
    fn inner_for<'o, K>(
        &mut self,
        key: &Option<Symbol>,
        var: &str,
        iter: impl Iterator<Item = (K, &'o Value)>,
        body: &[Stmt],
    ) -> Result<()>
    where
        K: Into<Value>,
    {
        self.push_scope();
        for (k, v) in iter {
            if let Some(keyvar) = key {
                self.set(keyvar, k);
            }
            self.set(var, v.clone());
            match self.block(&body) {
                Ok(_) => {}
                Err(e) => match e.kind {
                    ErrorKind::Jump(Jump::Break) => break,
                    ErrorKind::Jump(Jump::Continue) => continue,
                    _ => {
                        self.pop_scope();
                        return Err(e)
                    },
                },
            }
            self.scope().borrow_mut().clear();
        }
        self.pop_scope();
        Ok(())
    }

    /// Is the first stmt a <head> tag?
    fn first_is_head(&self, stmts: &[Stmt]) -> bool {
        if let Some(Stmt::Tag(tag)) = stmts.get(0) {
            if let Stmt::Value(Value::String(tag)) = &*tag.tag {
                return tag == "head";
            }
        }
        false
    }

    /// Is the given string a keyword or existing variable?
    fn is_keyword_or_var(&self, it: &str) -> bool {
        self.var_exists(it) || matches!(it, "return" | "break" | "continue")
    }
}
