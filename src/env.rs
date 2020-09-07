use {
    crate::{builtins, Value},
    std::collections::HashMap,
};

pub struct Env {
    map: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Env {
        let mut new = Env {
            map: HashMap::new(),
        };
        new.set("abc", vec!["a", "b", "c"]);
        new
    }

    pub fn set<S: AsRef<str>, V: Into<Value>>(&mut self, key: S, val: V) {
        self.map.insert(key.as_ref().to_string(), val.into());
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.map.get(name)
    }
}
