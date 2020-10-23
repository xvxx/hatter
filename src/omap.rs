//! Insertion-order preserving HashMap

use {
    crate::{Symbol, Value},
    std::collections::HashMap,
};

/// A HashMap for Hatter that preserves insertion order.
pub struct OMap {
    map: HashMap<Symbol, Value>,
    order: Vec<Symbol>,
}

impl OMap {
    pub fn new() -> OMap {
        OMap {
            map: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        debug_assert!(self.map.len() == self.order.len());
        self.map.len()
    }

    pub fn get<S: Into<Symbol>>(&self, key: S) -> Option<&Value> {
        self.map.get(&key.into())
    }

    pub fn insert<S: Into<Symbol>>(&mut self, key: S, val: Value) {
        debug_assert!(self.map.len() == self.order.len());
        let sym = key.into();
        self.map.insert(sym.clone(), val);
        self.order.push(sym);
    }

    pub fn remove<S: Into<Symbol>>(&mut self, key: S) {
        debug_assert!(self.map.len() == self.order.len());
        let sym = key.into();
        self.map.remove(&sym);
        self.order
            .remove(self.order.iter().position(|s| s == &sym).unwrap());
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Symbol, &Value)> {
        self.order
            .iter()
            .map(move |k| (k, self.map.get(k).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut map = OMap::new();
        map.insert("name", "Cyndi L.".into());
        assert_eq!("Cyndi L.", map.get(&Symbol::from("name")).unwrap().to_str());
        map.remove("name");
        assert_eq!(None, map.get(&Symbol::from("name")));
    }

    #[test]
    fn test_iter() {
        let mut map = OMap::new();
        map.insert("name", "Bilbo Baggins".into());
        map.insert("age", "111".into());
        let mut out = String::new();
        for (k, v) in map.iter() {
            out.push_str(&format!("{}: {}\n", k, v));
        }
        assert_eq!("name: Bilbo Baggins\nage: 111\n", out);
    }
}
