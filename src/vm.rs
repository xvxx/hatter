use {
    crate::{builtins, Builtin, Code, Result, Value},
    std::collections::{BTreeMap, HashMap},
};

pub type Env = HashMap<String, Value>;

pub struct VM {
    stack: Vec<Value>, // value stack
    calls: Vec<usize>, // call stack
    envs: Vec<Env>,
    ip: usize,
    out: String,
    builtins: HashMap<String, Builtin>,
}

pub fn run(inst: Vec<Code>) -> Result<String> {
    let mut vm = VM::new();
    vm.run(inst)?;
    Ok(vm.out)
}

impl VM {
    fn new() -> VM {
        VM {
            ip: 0,
            stack: vec![],
            calls: vec![],
            envs: vec![Env::new()],
            out: String::new(),
            builtins: builtins(),
        }
    }

    fn pop_call(&mut self) -> usize {
        self.calls.pop().unwrap_or(0)
    }

    fn pop_stack(&mut self) -> Value {
        self.stack.pop().unwrap_or(Value::None)
    }

    fn push<T: Into<Value>>(&mut self, v: T) {
        self.stack.push(v.into());
    }

    fn lookup(&self, key: &str) -> Option<&Value> {
        for env in &self.envs {
            if let Some(v) = env.get(key) {
                return Some(v);
            }
        }
        None
    }

    fn env(&mut self) -> &mut Env {
        let len = self.envs.len();
        &mut self.envs[len - 1]
    }

    fn set<S: AsRef<str>, V: Into<Value>>(&mut self, key: S, val: V) {
        self.env().insert(key.as_ref().to_string(), val.into());
    }

    fn run(&mut self, inst: Vec<Code>) -> Result<()> {
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
                    if !self.lookup(name).is_some() {
                        let val = self.pop_stack();
                        self.set(name, val);
                        self.ip += 1;
                    } else {
                        return error!("var already exists: {}", name);
                    }
                }
                Code::SetIfSet(name) => {
                    if self.lookup(name).is_some() {
                        let val = self.pop_stack();
                        self.set(name, val);
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
                Code::Return => self.ip = self.pop_call(),
                Code::Call(name, arity) => {
                    self.ip += 1;
                    let mut args = vec![];
                    for _ in 0..*arity {
                        args.push(self.pop_stack());
                    }
                    let args = args.into_iter().rev().collect::<Vec<_>>();
                    if let Some(f) = self.builtins.get(name) {
                        let retval = f(self, &args);
                        self.push(retval);
                    } else {
                        return error!("can't find fn named {}", name);
                    }
                }
            }
        }
        Ok(())
    }

    fn end_loop(&mut self) -> Result<()> {
        self.ip += 1;
        self.pop_stack(); // pop counter
        self.pop_stack(); // pop iterator/list
        self.envs.pop();
        Ok(())
    }

    fn init_loop(&mut self) -> Result<()> {
        self.ip += 1;
        self.envs.push(Env::new());
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
