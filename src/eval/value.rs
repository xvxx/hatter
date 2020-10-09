//! Value is the dynamic type that represents an active Hatter object,
//! including objects you define in your own code.

use {
    crate::{Result, Symbol},
    std::{fmt, rc::Rc},
};

mod from;
mod inner;
pub use inner::{Fn, List, Map, NativeFn, Object, SpecialFn};

/// Value is the dynamic type that represents an active Hatter object,
/// including objects you define in your own code.
#[derive(Clone)]
pub enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(Symbol),
    List(List),
    Map(Map),
    Fn(Fn),
    Object(Rc<dyn Object>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::String(s) => write!(f, "{}", s),
            Value::List(..) => write!(f, "{:?}", self),
            _ => write!(f, "{}", self.to_str()),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            None => write!(f, "None"),
            Bool(b) => write!(f, "{}", b),
            Number(num) => write!(f, "{}", num),
            String(s) => write!(f, r#""{}""#, s),
            Fn(..) => f.debug_struct("Function").field("val", &"?").finish(),
            List(list) => write!(
                f,
                "[{}]",
                &list
                    .borrow()
                    .iter()
                    .map(|i| format!("{:?}", i))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            Map(..) => f.debug_struct("Map").field("val", &"?").finish(),
            Object(..) => f.debug_struct("Object").field("val", &"?").finish(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Value::None => matches!(other, Value::None),
            Value::Fn(..) | Value::Object(..) => false,
            Value::Bool(true) => matches!(other, Value::Bool(true)),
            Value::Bool(false) => matches!(other, Value::Bool(false)),
            Value::Number(num) => {
                if let Value::Number(n2) = other {
                    num == n2
                } else {
                    false
                }
            }
            Value::String(s) => {
                if let Value::String(s2) = other {
                    s == s2
                } else {
                    false
                }
            }
            Value::List(list) => {
                if let Value::List(l2) = other {
                    list == l2
                } else {
                    false
                }
            }
            Value::Map(map) => {
                if let Value::Map(m2) = other {
                    map == m2
                } else {
                    false
                }
            }
        }
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        if let Value::String(s) = other {
            s == self
        } else {
            false
        }
    }
}

impl Value {
    pub fn ok(self) -> Result<Value> {
        Ok(self)
    }

    pub fn some(self) -> Option<Value> {
        Some(self)
    }

    pub fn len(&self) -> usize {
        match self {
            Value::List(list) => list.borrow().len(),
            Value::Map(map) => map.borrow().len(),
            Value::String(s) => s.len(),
            _ => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::None => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn to_f64(&self) -> f64 {
        if let Value::Number(n) = self {
            *n
        } else {
            0.0
        }
    }

    pub fn to_str(&self) -> &str {
        use Value::*;
        match self {
            None => "",
            String(s) => &s.to_str(),
            Number(..) => "(number)",
            Fn(..) => "{function}",
            List(..) => "(list)",
            Map(..) => "(map)",
            Object(..) => "(object)",
            Bool(b) => {
                if *b {
                    "true"
                } else {
                    "false"
                }
            }
        }
    }

    pub fn typename(&self) -> &str {
        use Value::*;
        match self {
            None => "None",
            Bool(..) => "Bool",
            Number(..) => "Number",
            String(..) => "String",
            Fn(..) => "Fn",
            List(..) => "List",
            Map(..) => "Map",
            Object(..) => "Object",
        }
    }
}
