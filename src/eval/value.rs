use {
    crate::{Args, Env, Result, Scope, Stmt, Symbol},
    std::{
        cell::RefCell,
        collections::{BTreeMap, HashMap},
        fmt,
        ops::Deref,
        rc::Rc,
    },
};

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

#[derive(Clone)]
pub enum Fn {
    Fn(Vec<Symbol>, Vec<Stmt>, Scope),
    Native(Rc<NativeFn>),
    Special(Rc<SpecialFn>),
}

pub type NativeFn = dyn std::ops::Fn(Args) -> Result<Value>;
pub type SpecialFn = dyn std::ops::Fn(&mut Env, &[Stmt]) -> Result<Value>;

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
                Value::String(item.to_string().into())
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
        Value::List(vec.into_iter().map(val).collect::<Vec<_>>().into())
    }
}

impl<T: Copy + Into<Value>> From<&Vec<T>> for Value {
    fn from(vec: &Vec<T>) -> Self {
        Value::List(vec.iter().map(|v| (*v).into()).collect::<Vec<_>>().into())
    }
}

impl<T: Copy + Into<Value>> From<&[T]> for Value {
    fn from(vec: &[T]) -> Self {
        Value::List(vec.iter().map(|v| (*v).into()).collect::<Vec<_>>().into())
    }
}

impl From<Symbol> for Value {
    fn from(item: Symbol) -> Self {
        Value::String(item)
    }
}

impl From<&Symbol> for Value {
    fn from(item: &Symbol) -> Self {
        Value::String(item.clone())
    }
}

impl From<&str> for Value {
    fn from(item: &str) -> Self {
        Value::String(item.into())
    }
}

impl From<&&str> for Value {
    fn from(item: &&str) -> Self {
        Value::String(Symbol::from(item))
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
            new.insert(k.as_ref().into(), v.into());
        }
        Value::Map(new.into())
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
            new.insert(k.as_ref().into(), val(v));
        }
        Value::Map(new.into())
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

impl<F> From<F> for Value
where
    F: 'static + std::ops::Fn(Args) -> Result<Value>,
{
    fn from(f: F) -> Value {
        Value::Fn(Fn::Native(rc!(f)))
    }
}

impl<V> From<(V,)> for Value
where
    V: Into<Value>,
{
    fn from(f: (V,)) -> Value {
        Value::List(vec![f.0.into()].into())
    }
}

impl<V1, V2> From<(V1, V2)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
{
    fn from(f: (V1, V2)) -> Value {
        Value::List(vec![f.0.into(), f.1.into()].into())
    }
}

impl<V1, V2, V3> From<(V1, V2, V3)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
{
    fn from(f: (V1, V2, V3)) -> Value {
        Value::List(vec![f.0.into(), f.1.into(), f.2.into()].into())
    }
}

impl<V1, V2, V3, V4> From<(V1, V2, V3, V4)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
    V4: Into<Value>,
{
    fn from(f: (V1, V2, V3, V4)) -> Value {
        Value::List(vec![f.0.into(), f.1.into(), f.2.into(), f.3.into()].into())
    }
}

impl<V1, V2, V3, V4, V5> From<(V1, V2, V3, V4, V5)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
    V4: Into<Value>,
    V5: Into<Value>,
{
    fn from(f: (V1, V2, V3, V4, V5)) -> Value {
        Value::List(vec![f.0.into(), f.1.into(), f.2.into(), f.3.into(), f.4.into()].into())
    }
}

impl<V1, V2, V3, V4, V5, V6> From<(V1, V2, V3, V4, V5, V6)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
    V4: Into<Value>,
    V5: Into<Value>,
    V6: Into<Value>,
{
    fn from(f: (V1, V2, V3, V4, V5, V6)) -> Value {
        Value::List(
            vec![
                f.0.into(),
                f.1.into(),
                f.2.into(),
                f.3.into(),
                f.4.into(),
                f.5.into(),
            ]
            .into(),
        )
    }
}
