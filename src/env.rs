use {
    crate::{builtins, Builtin, ErrorKind, Result, Stmt, Tag, Value},
    std::{
        collections::{BTreeMap, HashMap},
        ops,
        rc::Rc,
    },
};

/// Eval a list of Stmts and return the last's Value.
pub fn eval(stmts: &[Stmt]) -> Result<Value> {
    let mut env = Env::new();
    env.block(stmts)
}

/// Render Stmts to a String.
pub fn render(stmts: &[Stmt]) -> Result<String> {
    let mut env = Env::new();
    env.render(stmts)
}

/// Error-ish that lets us abort what we're doing.
#[derive(Debug, PartialEq)]
pub enum Jump {
    Break,
    Continue,
    Return(Value),
}

/// Name -> Val map
pub type Scope = HashMap<String, Value>;

pub struct Env {
    scopes: Vec<Scope>,
    helpers: HashMap<String, Rc<Builtin>>,
    out: String,
}

impl ops::Deref for Env {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        self.scope()
    }
}

impl ops::DerefMut for Env {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mut_scope()
    }
}

impl Env {
    pub fn new() -> Env {
        Env {
            scopes: vec![Scope::new()],
            helpers: builtins(),
            out: String::new(),
        }
    }

    /// Return and clear output.
    pub fn out(&mut self) -> String {
        std::mem::replace(&mut self.out, String::new())
    }

    /// Nearest (currently active) scope.
    pub fn scope(&self) -> &Scope {
        &self.scopes[self.scopes.len() - 1]
    }

    /// Add a new scope to the stack, which becomes active.
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
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

    /// Helper functions.
    #[allow(unused)]
    pub(crate) fn helpers(&self) -> &HashMap<String, Rc<Builtin>> {
        &self.helpers
    }

    /// Find a value in the nearest scope.
    pub fn lookup(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }

    /// Set a value to the nearest scope.
    pub fn set<V: Into<Value>>(&mut self, key: &str, val: V) {
        self.insert(key.to_string(), val.into());
    }

    /// Add a Rust function as a helper that can be invoked in templates.
    pub fn helper<S, F>(&mut self, key: S, f: F)
    where
        S: AsRef<str>,
        F: 'static + Fn(&mut Env, &[Value]) -> Value,
    {
        self.helpers.insert(key.as_ref().to_string(), rc!(f));
    }

    /// Print something. Includes trailing newline.
    pub fn print<V: Into<Value>>(&mut self, val: V) {
        self.out.push_str(&val.into().to_string());
        self.out.push('\n');
    }

    /// Render statements into a String.
    pub fn render(&mut self, stmts: &[Stmt]) -> Result<String> {
        self.block(stmts)?;
        Ok(self.out())
    }

    /// Evaluate a block of statements, returning the last one.
    pub fn block(&mut self, stmts: &[Stmt]) -> Result<Value> {
        let mut out = Value::None;
        for stmt in stmts {
            out = self.eval(&stmt)?;
            if !matches!(out, Value::None) {
                self.out.push_str(&out.to_string());
            }
        }
        Ok(out)
    }

    /// Evaluate one statement, returning its Value.
    fn eval(&mut self, stmt: &Stmt) -> Result<Value> {
        Ok(match stmt {
            Stmt::None => return error!("Stmt::None should never make it to eval()"),
            Stmt::Bool(x) => x.into(),
            Stmt::Number(x) => x.into(),
            Stmt::String(x) => x.into(),
            Stmt::Tag(x) => self.eval_tag(x)?,
            Stmt::List(x) => x
                .iter()
                .map(|v| self.eval(&v))
                .collect::<Result<Vec<_>>>()?
                .into(),
            Stmt::Map(pairs) => {
                let mut map = BTreeMap::new();
                for (k, v) in pairs {
                    map.insert(k, self.eval(v)?);
                }
                Value::from(map)
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
            Stmt::Call(name, args) => {
                if let Some(Value::Fn { params, body }) = self.lookup(&name) {
                    let params = params.clone();
                    let body = body.clone();
                    self.push_scope();
                    for (i, a) in args.iter().enumerate() {
                        if let Some(name) = params.get(i) {
                            let val = self.eval(&a)?;
                            self.set(name, val);
                        }
                    }
                    let out = self.block(&body)?;
                    self.pop_scope();
                    out
                } else if let Some(f) = self.helpers.get(name) {
                    let f = f.clone();
                    let args = args
                        .iter()
                        .map(|a| self.eval(&a))
                        .collect::<Result<Vec<_>>>()?;
                    f(self, &args)
                } else {
                    return error!("can't find fn: {}", name);
                }
            }
            Stmt::Return(expr) => return jump!(Jump::Return(self.eval(expr)?)),
            Stmt::If(conds) => {
                let mut out = Value::None;
                for (test, body) in conds {
                    if self.eval(test)?.to_bool() {
                        out = self.block(body)?;
                        break;
                    }
                }
                out
            }
            Stmt::For(..) => self.eval_for(stmt)?,
            Stmt::Assign(name, expr, is_reassign) => {
                let exists = self.contains_key(name);
                if exists && !is_reassign {
                    return error!("{} already set", name);
                } else if !exists && *is_reassign {
                    return error!("{} is not set", name);
                }
                let val = self.eval(expr)?;
                self.set(name, val);
                Value::None
            }
            Stmt::Fn(params, body) => Value::Fn {
                params: params.clone(),
                body: body.clone(),
            },
            Stmt::Args(..) => unimplemented!(),
        })
    }

    fn eval_tag(&mut self, _tag: &Tag) -> Result<Value> {
        Ok(Value::None)
    }

    /// Evaluate a for loop.
    fn eval_for(&mut self, stmt: &Stmt) -> Result<Value> {
        if let Stmt::For(key, val, expr, body) = stmt {
            match self.eval(&expr)? {
                Value::List(list) => self.inner_for(key, val, list.iter().enumerate(), body)?,
                Value::Map(map) => self.inner_for(key, val, map.iter(), body)?,
                v => return error!("expected List or Map, got {:?}", v),
            }
        }

        Ok(Value::None)
    }

    /// Shared "inner" for loop, over both maps and lists.
    fn inner_for<'o, K>(
        &mut self,
        key: &Option<String>,
        var: &str,
        iter: impl Iterator<Item = (K, &'o Value)>,
        body: &[Stmt],
    ) -> Result<()>
    where
        K: Into<Value>,
    {
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
                    _ => return Err(e),
                },
            }
        }
        Ok(())
    }
}
