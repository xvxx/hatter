use {
    crate::{Code, Env, Result, Value},
    std::collections::HashMap,
};

struct VM<'p> {
    stack: Vec<Value>, // value stack
    calls: Vec<usize>, // call stack
    env: Env<'p>,
    ip: usize,
    out: String,
    builtins: HashMap<String, fn()>,
}

pub fn run(inst: Vec<Code>) -> Result<String> {
    let mut vm = VM::new();
    vm.run(inst)?;
    Ok(vm.out)
}

impl<'p> VM<'p> {
    fn new() -> VM<'p> {
        VM {
            ip: 0,
            stack: vec![],
            calls: vec![],
            env: Env::root(),
            out: String::new(),
            builtins: HashMap::new(),
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

    fn run(&mut self, inst: Vec<Code>) -> Result<()> {
        while let Some(inst) = inst.get(self.ip) {
            match inst {
                Code::Noop => self.ip += 1,
                Code::JumpTo(n) => self.ip = *n,
                Code::JumpBy(n) => self.ip = (self.ip as isize + n) as usize,
                Code::JumpIfTrue(n) => {
                    if self.pop_stack().to_bool() {
                        self.ip = (self.ip as isize + n) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
                Code::JumpIfFalse(n) => {
                    if !self.pop_stack().to_bool() {
                        self.ip = (self.ip as isize + n) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
                Code::Exit => return Ok(()),
                Code::Print => {
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
                Code::Incr(name) => {
                    if let Some(Value::Number(n)) = self.env.lookup(name) {
                        let n = *n;
                        self.env.set(name, n + 1.0);
                    }
                    self.ip += 1;
                }
                Code::Decr(name) => {
                    if let Some(Value::Number(n)) = self.env.lookup(name) {
                        let n = *n;
                        self.env.set(name, n - 1.0);
                    }
                    self.ip += 1;
                }
                Code::ShouldLoop => {
                    if let Value::Number(n) = self.pop_stack() {
                        if let Value::List(list) = self.pop_stack() {
                            self.push((n as usize) < list.len());
                        } else {
                            return error!("ShouldLoop expected List at -2 of stack");
                        }
                    } else {
                        return error!("ShouldLoop expected Number at top of stack");
                    }
                }
                Code::Loop(key, val) => {
                    let iter = self.pop_stack();
                    match iter {
                        Value::List(list) => {
                            if let Some(k) = key {
                                self.env.set(k, 0);
                            }
                            self.env.set(val, list[0].clone());
                            self.push(list);
                            self.push(0);
                        }
                        Value::Number(n) => {
                            let new = n + 1.0;
                            if let Value::List(list) = self.pop_stack() {
                                if let Some(k) = key {
                                    self.env.set(k, new);
                                }
                                self.env.set(val, list[new as usize].clone());
                                self.push(list);
                                self.push(new);
                            } else {
                                return error!("expected Number on top of stack");
                            }
                        }
                        Value::Map(map) => {
                            if let Some(fst) = map.keys().next() {
                                if let Some(k) = key {
                                    self.env.set(k, fst);
                                }
                                self.env
                                    .set(val, map.get(fst).unwrap_or(&Value::None).clone());
                                let fst = Value::from(fst);
                                self.push(map);
                                self.push(fst);
                            }
                        }
                        Value::String(s) => {
                            let next = s;
                            if let Value::Map(map) = self.pop_stack() {
                                if let Some((keyname, v)) = map.range(..next).next() {
                                    let keyname = Value::from(keyname);
                                    if let Some(k) = key {
                                        self.env.set(k, keyname.clone());
                                    }
                                    self.env.set(val, v.clone());
                                    self.push(map);
                                    self.push(keyname);
                                }
                            } else {
                                return error!("expected String on top of stack");
                            }
                        }
                        _ => return error!("can only loop over List or Map, got {:?}", iter),
                    }
                }
                Code::Lookup(name) => {
                    if let Some(v) = self.env.lookup(name) {
                        let v = v.clone();
                        self.push(v);
                    }
                    self.ip += 1;
                }
                Code::Set(name) => {
                    let val = self.pop_stack();
                    self.env.set(name, val);
                    self.ip += 1;
                }
                Code::Return => self.ip = self.pop_call(),
                Code::Call(name, arity) => {
                    let mut args = vec![];
                    for _ in 0..*arity {
                        args.push(self.pop_stack());
                    }
                    let args = args.into_iter().rev().collect::<Vec<_>>();
                    if let Some(Value::Fn(f)) = self.env.lookup(name) {
                        self.calls.push(self.ip + 1);
                        let retval = f(&mut self.env, &args);
                        self.push(retval);
                    } else {
                        return error!("can't find fn named {}", name);
                    }
                }
            }
        }
        Ok(())
    }
}
