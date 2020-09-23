use {
    crate::{Args, Result, Stmt},
    std::{
        collections::{BTreeMap, HashMap},
        fmt,
        rc::Rc,
    },
};

pub type Builtin = dyn Fn(Args) -> Result<Value>;

#[derive(Clone)]
pub enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Fn {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Object(Rc<dyn Object>),
}

#[allow(unused_variables)]
pub trait Object {
    fn get(&self, key: &str) -> Option<Value> {
        Some(Value::None)
    }

    fn set(&self, key: &str, val: Value) {}
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
            Fn { .. } => f.debug_struct("Function").field("val", &"?").finish(),
            List(list) => write!(
                f,
                "[{}]",
                &list
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
            Value::None | Value::Fn { .. } | Value::Object(..) => false,
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

impl Value {
    pub fn ok(self) -> Result<Value> {
        Ok(self)
    }

    pub fn len(&self) -> usize {
        match self {
            Value::List(list) => list.len(),
            Value::Map(map) => map.len(),
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
            String(s) => &s,
            Number(..) => "(number)",
            Fn { .. } => "{function}",
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
            Fn { .. } => "Fn",
            List(..) => "List",
            Map(..) => "Map",
            Object(..) => "Object",
        }
    }
}

fn val<T: Into<Value>>(i: T) -> Value {
    i.into()
}

macro_rules! into_bool {
    ($type:ty) => {
        impl From<$type> for Value {
            fn from(item: $type) -> Self {
                Value::Bool(item.clone())
            }
        }
    };
}

macro_rules! into_string {
    ($type:ty) => {
        impl From<$type> for Value {
            fn from(item: $type) -> Self {
                Value::String(item.to_string())
            }
        }
    };
}

macro_rules! into_number {
    ($type:ty) => {
        impl From<$type> for Value {
            fn from(item: $type) -> Self {
                Value::Number(item.clone().into())
            }
        }
    };
}

macro_rules! into_number_as {
    ($type:ty, $conv:ty) => {
        impl From<$type> for Value {
            fn from(item: $type) -> Self {
                Value::Number((item.clone() as $conv).into())
            }
        }
    };
}

into_string!(&str);
into_string!(&&str);
into_string!(String);
into_string!(&String);
into_string!(&&String);

into_number!(i32);
into_number!(&i32);
into_number!(f64);
into_number!(&f64);
into_number_as!(usize, i32);
into_number_as!(&usize, i32);
into_number_as!(isize, i32);
into_number_as!(&isize, i32);

into_bool!(bool);
into_bool!(&bool);

impl From<&Value> for Value {
    fn from(v: &Value) -> Value {
        v.clone()
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(vec: Vec<T>) -> Self {
        Value::List(vec.into_iter().map(val).collect())
    }
}

impl<T: Copy + Into<Value>> From<&Vec<T>> for Value {
    fn from(vec: &Vec<T>) -> Self {
        Value::List(vec.iter().map(|v| (*v).into()).collect())
    }
}

impl<T: Copy + Into<Value>> From<&[T]> for Value {
    fn from(vec: &[T]) -> Self {
        Value::List(vec.iter().map(|v| (*v).into()).collect())
    }
}

impl<S, V> From<BTreeMap<S, V>> for Value
where
    S: AsRef<str>,
    V: Into<Value>,
{
    fn from(map: BTreeMap<S, V>) -> Self {
        let mut new = BTreeMap::new();
        for (k, v) in map {
            new.insert(k.as_ref().to_string(), v.into());
        }
        Value::Map(new)
    }
}

impl<S, V> From<HashMap<S, V>> for Value
where
    S: AsRef<str>,
    V: Into<Value>,
{
    fn from(map: HashMap<S, V>) -> Self {
        let mut new = BTreeMap::new();
        for (k, v) in map {
            new.insert(k.as_ref().to_string(), val(v));
        }
        Value::Map(new)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(o: Option<T>) -> Self {
        if o.is_none() {
            Value::None
        } else {
            o.unwrap().into()
        }
    }
}

impl From<Vec<(String, Stmt)>> for Value {
    fn from(pairs: Vec<(String, Stmt)>) -> Value {
        let mut map: BTreeMap<String, Value> = BTreeMap::new();
        for (k, v) in pairs {
            map.insert(k, Value::from(v));
        }
        map.into()
    }
}

impl From<Stmt> for Value {
    fn from(s: Stmt) -> Value {
        match s {
            Stmt::Bool(x) => Value::from(x),
            Stmt::Number(x) => Value::from(x),
            Stmt::String(x) => Value::from(x),
            Stmt::List(x) => Value::from(x),
            Stmt::Map(x) => Value::from(x),
            _ => unimplemented!(),
        }
    }
}
