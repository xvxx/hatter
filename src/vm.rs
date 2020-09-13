use {
    crate::{builtins, Builtin, Code, Result, Value},
    std::collections::{BTreeMap, HashMap},
};

pub type Scope = HashMap<String, Value>;

pub struct VM {
    strict: bool,       // strict mode?
    stack: Vec<Value>,  // value stack
    calls: Vec<usize>,  // call stack
    scopes: Vec<Scope>, // stack of scopes
    tags: Vec<String>,  // track tags to auto-close
    ip: usize,          // instruction pointer
    builtins: HashMap<String, Builtin>,
}

pub fn run(inst: Vec<Code>) -> Result<()> {
    let mut vm = VM::new(false);
    vm.run(inst)?;
    Ok(())
}

impl VM {
    pub fn new(strict: bool) -> VM {
        VM {
            strict,
            ip: 0,
            stack: vec![],
            calls: vec![],
            tags: vec![],
            scopes: vec![Scope::new()],
            builtins: builtins(),
        }
    }

    fn pop_call(&mut self) -> usize {
        self.calls.pop().unwrap_or(0)
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::None)
    }

    fn pop_tag(&mut self) -> String {
        self.tags.pop().unwrap_or_else(|| "".into())
    }

    fn push_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    fn push<T: Into<Value>>(&mut self, v: T) {
        self.stack.push(v.into());
    }

    fn lookup(&self, key: &str) -> Option<&Value> {
        for scope in &self.scopes {
            if let Some(v) = scope.get(key) {
                return Some(v);
            }
        }
        None
    }

    fn scope(&mut self) -> &mut Scope {
        let len = self.scopes.len();
        &mut self.scopes[len - 1]
    }

    fn find_scope(&mut self, key: &str) -> Option<&mut Scope> {
        for (i, scope) in self.scopes.iter().enumerate() {
            if scope.contains_key(key) {
                return Some(&mut self.scopes[i]);
            }
        }
        None
    }

    fn set<S: AsRef<str>, V: Into<Value>>(&mut self, key: S, val: V) {
        self.scope().insert(key.as_ref().to_string(), val.into());
    }

    pub fn run(&mut self, inst: Vec<Code>) -> Result<()> {
        self.ip = 0;
        while let Some(inst) = inst.get(self.ip) {
            // println!("\n>> VM: {:?}\nSTACK: {:?}\n", inst, self.stack);
            match inst {
                Code::Break | Code::Continue => {
                    panic!("{:?} should be handled in the compiler", inst)
                }
                Code::Debug(..) | Code::Label(..) => self.ip += 1,
                Code::JumpTo(_) | Code::JumpToIfTrue(_) | Code::JumpToIfFalse(_) => {
                    self.ip += 1;
                }
                Code::JumpBy(_, n) => self.ip = (self.ip as isize + n) as usize,
                Code::JumpByIfTrue(_, n) => {
                    if self.pop_stack().to_bool() {
                        self.ip = (self.ip as isize + n) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
                Code::JumpByIfFalse(_, n) => {
                    if !self.pop_stack().to_bool() {
                        self.ip = (self.ip as isize + n) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
                Code::Exit => return Ok(()),
                Code::Print(v) => {
                    println!("{}", v.to_string());
                    self.ip += 1;
                }
                Code::PrintPop => {
                    println!("{}", self.pop_stack().to_string());
                    self.ip += 1;
                }
                Code::Push(v) => {
                    self.push(v.clone());
                    self.ip += 1;
                }
                Code::Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
                Code::TestShouldLoop => {
                    self.ip += 1;
                    if let Value::Number(n) = self.pop_stack() {
                        if let Value::List(list) = self.pop_stack() {
                            let len = list.len();
                            self.push(list);
                            self.push(1.0 + n);
                            self.push((1 + n as usize) < len);
                        } else {
                            return error!(
                                "TestShouldLoop expected List at -2 of stack, stack: {:?}",
                                self.stack
                            );
                        }
                    } else {
                        return error!(
                            "TestShouldLoop expected Number at top of stack, stack: {:?}",
                            self.stack
                        );
                    }
                }
                Code::InitLoop => self.init_loop()?,
                Code::EndLoop => self.end_loop()?,
                Code::Loop(key, val) => self.do_loop(key, val)?,
                Code::PrintVar(name) => {
                    if let Some(v) = self.lookup(name) {
                        println!("{}", v);
                    } else if self.strict {
                        return error!("can't find {}", name);
                    } else {
                        println!("{}", name);
                    }
                    self.ip += 1;
                }
                Code::Lookup(name) => {
                    if let Some(v) = self.lookup(name) {
                        let v = v.clone();
                        self.push(v);
                    } else {
                        return error!("can't find {}", name);
                    }
                    self.ip += 1;
                }
                Code::Set(name) => {
                    if self.scope().get(name).is_none() {
                        let val = self.pop_stack();
                        self.set(name, val);
                        self.ip += 1;
                    } else {
                        return error!("var already exists: {}", name);
                    }
                }
                Code::SetIfSet(name) => {
                    let val = self.pop_stack();
                    if let Some(scope) = self.find_scope(name) {
                        scope.insert(name.to_string(), val);
                        self.ip += 1;
                    } else {
                        return error!("var doesn't exist: {}", name);
                    }
                }
                Code::List(len) => {
                    self.ip += 1;
                    let list: Vec<_> = self.stack.drain(self.stack.len() - len..).collect();
                    self.push(list);
                }
                Code::Map(len) => {
                    self.ip += 1;
                    let mut map = BTreeMap::new();
                    let (keys, values): (Vec<_>, Vec<_>) = self
                        .stack
                        .drain(self.stack.len() - (len * 2)..)
                        .enumerate()
                        .partition(|(i, _)| i % 2 == 0);
                    for (i, (_, k)) in keys.iter().enumerate() {
                        let (_, v) = &values[i];
                        map.insert(k.to_string(), v.clone());
                    }
                    self.push(Value::Map(map));
                }
                Code::Tag(len) => {
                    self.ip += 1;
                    let mut out = vec![];
                    let (keys, values): (Vec<_>, Vec<_>) = self
                        .stack
                        .drain(self.stack.len() - (len * 2)..)
                        .enumerate()
                        .partition(|(i, _)| i % 2 == 0);
                    let closed = matches!(self.pop_stack(), Value::Bool(true));
                    let name = self.pop_stack().to_string();
                    if name == "head" && self.tags.is_empty() {
                        out.push("<!DOCTYPE html>\n<html>".into());
                        self.tags.push("html".into());
                    }
                    out.push(format!("<{}", name));
                    if !closed {
                        self.push_tag(name);
                    }
                    let mut classes = vec![];
                    for (i, (_, k)) in keys.iter().enumerate() {
                        let (_, v) = &values[i];
                        if !matches!(v, Value::Bool(false) | Value::None) {
                            if let Value::String(s) = k {
                                if s == "class" {
                                    classes.push(v.to_string());
                                    continue;
                                }
                            }
                            out.push(format!("{}='{}'", k, v));
                        }
                    }
                    if !classes.is_empty() {
                        out.push(format!("class='{}'", classes.join(" ")));
                    }
                    if closed {
                        out.push("/".into());
                    }
                    println!("{}>", out.join(" "));
                }
                Code::CloseTag => {
                    self.ip += 1;
                    println!("</{}>", self.pop_tag());
                }
                Code::Return => self.ip = self.pop_call(),
                Code::Call(name, arity) => {
                    self.ip += 1;
                    let mut args = vec![];
                    for _ in 0..*arity {
                        args.push(self.pop_stack());
                    }
                    let mut args = args.into_iter().rev();
                    if let Some(Value::Fn { params, body }) = self.lookup(name) {
                        let mut scope = Scope::new();
                        for param in params {
                            if let Some(arg) = args.next() {
                                scope.insert(param.clone(), arg);
                            } else {
                                break;
                            }
                        }
                        let body = body.clone();
                        let ip = self.ip;
                        self.scopes.push(scope);
                        self.run(body)?;
                        self.scopes.pop();
                        self.ip = ip;
                    } else if let Some(f) = self.builtins.get(name) {
                        let retval = f(self, &args.collect::<Vec<_>>());
                        self.push(retval);
                    } else {
                        return error!("can't find fn named {}", name);
                    }
                }
            }
        }
        if !self.tags.is_empty() {
            for tag in self.tags.drain(..) {
                println!("</{}>", tag);
            }
        }
        Ok(())
    }

    fn end_loop(&mut self) -> Result<()> {
        self.ip += 1;
        self.pop_stack(); // pop counter
        self.pop_stack(); // pop iterator/list
        self.scopes.pop();
        Ok(())
    }

    fn init_loop(&mut self) -> Result<()> {
        self.ip += 1;
        self.scopes.push(Scope::new());
        let iter = self.pop_stack();
        match iter {
            Value::List(list) => {
                self.push(list);
                self.push(0);
            }
            Value::Map(map) => {
                if let Some(fst) = map.keys().next() {
                    let fst = Value::from(fst);
                    self.push(map);
                    self.push(fst);
                }
            }
            _ => return error!("can only loop over List or Map, got {:?}", iter),
        }
        Ok(())
    }

    fn do_loop(&mut self, key: &Option<String>, val: &str) -> Result<()> {
        self.ip += 1;
        let iter = self.pop_stack();
        match iter {
            Value::Number(n) => {
                if let Value::List(list) = self.pop_stack() {
                    if let Some(k) = key {
                        self.set(k, n);
                    }
                    self.set(val, list[n as usize].clone());
                    self.push(list);
                    self.push(n);
                } else {
                    return error!("expected Number on top of stack, stack: {:?}", self.stack);
                }
            }
            Value::String(s) => {
                let next = s;
                if let Value::Map(map) = self.pop_stack() {
                    if let Some((keyname, v)) = map.range(..next).next() {
                        let keyname = Value::from(keyname);
                        if let Some(k) = key {
                            self.set(k, keyname.clone());
                        }
                        self.set(val, v.clone());
                        self.push(map);
                        self.push(keyname);
                    }
                } else {
                    return error!("expected String on top of stack, stack: {:?}", self.stack);
                }
            }
            _ => return error!("can only loop over List or Map, got {:?}", iter),
        }
        Ok(())
    }
}
