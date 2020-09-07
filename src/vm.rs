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

    fn run(&mut self, inst: Vec<Code>) -> Result<()> {
        while let Some(inst) = inst.get(self.ip) {
            match inst {
                Code::Noop => self.ip += 1,
                Code::JumpTo(n) => self.ip = *n,
                Code::JumpBy(n) => self.ip += n,
                Code::JumpIfTrue(n) => {
                    if self.pop_stack().to_bool() {
                        self.ip += n;
                    } else {
                        self.ip += 1;
                    }
                }
                Code::JumpIfFalse(n) => {
                    if !self.pop_stack().to_bool() {
                        self.ip += n;
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
                    self.stack.push(v.clone());
                    self.ip += 1;
                }
                Code::Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
                Code::Lookup(name) => {
                    if let Some(v) = self.env.lookup(name) {
                        self.stack.push(v.clone());
                    }
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
                        self.stack.push(f(&mut self.env, &args));
                    } else {
                        return error!("can't find fn named {}", name);
                    }
                }
            }
        }
        Ok(())
    }
}
