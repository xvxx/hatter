use {
    crate::{Code, VM},
    std::{
        collections::{BTreeMap, HashMap},
        fmt,
        rc::Rc,
    },
};

pub type Builtin = dyn Fn(&mut VM, &[Value]) -> Value;

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
        body: Vec<Code>,
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
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            None => f.debug_struct("None").finish(),
            Bool(b) => f.debug_struct(if *b { "True" } else { "False" }).finish(),
            Number(num) => f.debug_struct("Number").field("val", &num).finish(),
            String(s) => f.debug_struct("String").field("val", &s).finish(),
            Fn { .. } => f.debug_struct("Function").field("val", &"?").finish(),
            List(list) => f
                .debug_struct("List")
                .field(
                    "val",
                    &list
                        .iter()
                        .map(|i| format!("{:?}", i))
                        .collect::<Vec<_>>()
                        .join(", "),
                )
                .finish(),
            Map(..) => f.debug_struct("Map").field("val", &"?").finish(),
            Object(..) => f.debug_struct("Object").field("val", &"?").finish(),
        }
    }
}

impl Value {
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

    pub fn to_string(&self) -> String {
        if let Value::Number(num) = self {
            format!("{}", num)
        } else {
            self.to_str().to_string()
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

into_bool!(bool);
into_bool!(&bool);

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
