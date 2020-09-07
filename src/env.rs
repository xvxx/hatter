use {
    crate::{builtins, Value},
    std::collections::HashMap,
};

pub struct Env<'p> {
    out: String,
    env: HashMap<String, Value>,
    builtins: HashMap<String, Value>,
    parent: Option<&'p Env<'p>>,
}

impl<'p> Env<'p> {
    pub fn root() -> Env<'p> {
        let mut new = Env::new();
        new.builtins = builtins();
        new
    }

    pub fn new() -> Env<'p> {
        Env {
            out: String::new(),
            env: HashMap::new(),
            builtins: HashMap::new(),
            parent: None,
        }
    }

    pub fn from(env: &'p Env) -> Env<'p> {
        let mut new = Env::new();
        new.parent = Some(env);
        new
    }

    pub fn set<S: AsRef<str>>(&mut self, key: S, val: Value) {
        self.env.insert(key.as_ref().to_string(), val);
    }

    pub fn putc(&mut self, c: char) {
        self.out.push(c);
    }

    pub fn print<S: AsRef<str>>(&mut self, it: S) {
        self.out.push_str(it.as_ref());
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
        if let Some(v) = self.env.get(name) {
            Some(v)
        } else if let Some(p) = self.parent {
            p.lookup(name)
        } else if let Some(b) = self.builtins.get(name) {
            Some(b)
        } else {
            None
        }
    }

    pub fn to_string(self) -> String {
        self.out
    }
}
