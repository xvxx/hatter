use {
    crate::{Args, Fn, OMap, Result, Stmt, Symbol, Value},
    std::{
        collections::{BTreeMap, HashMap},
        rc::Rc,
    },
};

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
    S: Into<Symbol>,
    V: Into<Value>,
{
    fn from(map: BTreeMap<S, V>) -> Self {
        let mut new = OMap::new();
        for (k, v) in map {
            new.insert(k.into(), v.into());
        }
        Value::Map(new.into())
    }
}

impl<S, V> From<HashMap<S, V>> for Value
where
    S: Into<Symbol>,
    V: Into<Value>,
{
    fn from(map: HashMap<S, V>) -> Self {
        let mut new = OMap::new();
        for (k, v) in map {
            new.insert(k.into(), val(v));
        }
        Value::Map(new.into())
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(o: Option<T>) -> Self {
        if let Some(v) = o {
            v.into()
        } else {
            Value::None
        }
    }
}

impl From<Stmt> for Value {
    fn from(s: Stmt) -> Value {
        match s {
            Stmt::Value(v) => v,
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
