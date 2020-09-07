use {crate::Env, std::collections::HashMap};

pub type HatterFn = fn(&mut Env, &[Value]) -> Value;

#[derive(Clone)]
pub enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Fn(HatterFn),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Value {
    pub fn to_string(&self) -> String {
        use Value::*;
        match self {
            None => "".to_string(),
            Bool(b) => b.to_string(),
            Number(num) => format!("{}", num),
            String(s) => s.clone(),
            Fn(..) => "{function}".to_string(),
            List(..) => "(list)".to_string(),
            Map(..) => "(map)".to_string(),
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
        }
    }
}

fn val<T: Into<Value>>(i: T) -> Value {
    i.into()
}

impl From<&str> for Value {
    fn from(item: &str) -> Self {
        Value::String(item.into())
    }
}

impl From<&&str> for Value {
    fn from(item: &&str) -> Self {
        Value::String(item.to_string())
    }
}

impl From<String> for Value {
    fn from(item: String) -> Self {
        Value::String(item)
    }
}
impl From<&String> for Value {
    fn from(item: &String) -> Self {
        Value::String(item.clone())
    }
}

impl From<usize> for Value {
    fn from(item: usize) -> Self {
        Value::from(item as i32)
    }
}

impl From<&usize> for Value {
    fn from(item: &usize) -> Self {
        Value::from(*item as i32)
    }
}

impl From<i32> for Value {
    fn from(item: i32) -> Self {
        Value::Number(item.into())
    }
}

impl From<&i32> for Value {
    fn from(item: &i32) -> Self {
        Value::from(*item)
    }
}

impl From<bool> for Value {
    fn from(item: bool) -> Self {
        Value::Bool(item)
    }
}

impl From<&bool> for Value {
    fn from(item: &bool) -> Self {
        Value::Bool(*item)
    }
}

impl From<Vec<Value>> for Value {
    fn from(vec: Vec<Value>) -> Self {
        Value::List(vec)
    }
}

impl From<&Vec<bool>> for Value {
    fn from(item: &Vec<bool>) -> Self {
        Value::List(item.iter().map(val).collect())
    }
}

impl From<Vec<bool>> for Value {
    fn from(item: Vec<bool>) -> Self {
        Value::List(item.iter().map(val).collect())
    }
}

impl From<&[&str]> for Value {
    fn from(item: &[&str]) -> Self {
        Value::List(item.iter().map(val).collect())
    }
}

impl From<Vec<&str>> for Value {
    fn from(item: Vec<&str>) -> Self {
        Value::List(item.iter().map(val).collect())
    }
}

impl From<&[String]> for Value {
    fn from(item: &[String]) -> Self {
        Value::List(item.iter().map(val).collect())
    }
}

impl From<Vec<String>> for Value {
    fn from(item: Vec<String>) -> Self {
        Value::List(item.iter().map(val).collect())
    }
}

impl From<Vec<i32>> for Value {
    fn from(item: Vec<i32>) -> Self {
        Value::List(item.iter().map(val).collect())
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(map: HashMap<String, Value>) -> Self {
        Value::Map(map)
    }
}

impl From<HashMap<String, &str>> for Value {
    fn from(map: HashMap<String, &str>) -> Self {
        let mut new = HashMap::new();
        for (k, v) in map {
            new.insert(k, val(v));
        }
        Value::Map(new)
    }
}

impl From<HashMap<&str, &str>> for Value {
    fn from(map: HashMap<&str, &str>) -> Self {
        let mut new = HashMap::new();
        for (k, v) in map {
            new.insert(k.to_string(), val(v));
        }
        Value::Map(new)
    }
}

impl From<HashMap<&str, i32>> for Value {
    fn from(map: HashMap<&str, i32>) -> Self {
        let mut new = HashMap::new();
        for (k, v) in map {
            new.insert(k.into(), val(v));
        }
        Value::Map(new)
    }
}

impl From<HashMap<&str, bool>> for Value {
    fn from(map: HashMap<&str, bool>) -> Self {
        let mut new = HashMap::new();
        for (k, v) in map {
            new.insert(k.into(), val(v));
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