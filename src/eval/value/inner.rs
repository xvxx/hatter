//! Value data structures more complicated than the basics are defined
//! here.

use {
    crate::{Args, Env, Result, Scope, Stmt, Symbol, Value},
    std::{cell::RefCell, collections::BTreeMap, ops::Deref, rc::Rc},
};

#[derive(Clone)]
pub struct List(Rc<RefCell<Vec<Value>>>);
impl List {
    pub fn new(s: Vec<Value>) -> Self {
        Self(rcell!(s))
    }
}
impl Deref for List {
    type Target = Rc<RefCell<Vec<Value>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PartialEq for List {
    fn eq(&self, other: &List) -> bool {
        let me = self.borrow();
        let you = other.borrow();
        me.len() == you.len() && me.iter().enumerate().all(|(i, v)| v == &you[i])
    }
}
impl From<Vec<Value>> for List {
    fn from(v: Vec<Value>) -> Self {
        Self::new(v)
    }
}

#[derive(Clone)]
pub struct Map(Rc<RefCell<BTreeMap<Symbol, Value>>>);
impl Map {
    pub fn new(m: BTreeMap<Symbol, Value>) -> Self {
        Self(rcell!(m))
    }
}
impl Deref for Map {
    type Target = Rc<RefCell<BTreeMap<Symbol, Value>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PartialEq for Map {
    fn eq(&self, other: &Map) -> bool {
        let me = self.borrow();
        let you = other.borrow();
        me.len() == you.len() && me.iter().all(|(i, v)| Some(v) == you.get(i))
    }
}
impl From<BTreeMap<Symbol, Value>> for Map {
    fn from(m: BTreeMap<Symbol, Value>) -> Self {
        Self::new(m)
    }
}

/// Three types of functions in Hatter:
/// Fn: Function defined in Hatter.
/// Native: Function defined in Rust.
/// Special: Function defined in Rust whose arguments aren't evaluated,
/// like a macro.
#[derive(Clone)]
pub enum Fn {
    Fn(Vec<Symbol>, Vec<Stmt>, Scope),
    Native(Rc<Native>),
    Special(Rc<Special>),
}

/// Hatter function defined in Rust.
pub type Native = dyn std::ops::Fn(Args) -> Result<Value>;

/// Hatter function defined in Rust whose arguments aren't evaluated,
/// like a macro.
pub type Special = dyn std::ops::Fn(&mut Env, &[Stmt]) -> Result<Value>;

/// If you want to expose your own structs to Hatter code, this is the
/// trait for you.
#[allow(unused_variables)]
pub trait Object {
    /// String name of this type, like "string" or "Person".
    fn typename(&self) -> &str;

    /// obj.KEY
    fn get(&self, key: &str) -> Option<Value> {
        None
    }

    /// obj.KEY = VAL
    fn set(&self, key: &str, val: Value) {}
}
