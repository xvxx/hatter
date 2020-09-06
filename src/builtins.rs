use {
    crate::{Env, Value},
    std::collections::HashMap,
};

pub fn builtins() -> HashMap<String, Value> {
    let mut map: HashMap<String, Value> = HashMap::new();
    fn add(_: &mut Env, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a + b);
            }
        }
        Value::None
    }
    fn sub(_: &mut Env, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a - b);
            }
        }
        Value::None
    }
    fn mul(_: &mut Env, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a * b);
            }
        }
        Value::None
    }
    fn div(_: &mut Env, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a / b);
            }
        }
        Value::None
    }
    fn print(env: &mut Env, args: &[Value]) -> Value {
        for arg in args {
            match arg {
                Value::None => env.print("None"),
                Value::String(s) => env.print(s),
                Value::Number(n) => env.print(n.to_string()),
                Value::Bool(b) => env.print(b.to_string()),
                _ => unimplemented!(),
            }
        }
        Value::None
    }
    fn to_uppercase(_env: &mut Env, args: &[Value]) -> Value {
        if let Some(Value::String(s)) = args.get(0) {
            Value::String(s.to_uppercase())
        } else {
            Value::String("Expected String".to_string())
        }
    }
    fn to_lowercase(_env: &mut Env, args: &[Value]) -> Value {
        if let Some(Value::String(s)) = args.get(0) {
            Value::String(s.to_lowercase())
        } else {
            Value::String("Expected String".to_string())
        }
    }

    map.insert("add".to_string(), Value::Fn(add));
    map.insert("sub".to_string(), Value::Fn(sub));
    map.insert("mul".to_string(), Value::Fn(mul));
    map.insert("div".to_string(), Value::Fn(div));
    map.insert("print".to_string(), Value::Fn(print));
    map.insert("to-uppercase".to_string(), Value::Fn(to_uppercase));
    map.insert("to-lowercase".to_string(), Value::Fn(to_lowercase));
    map
}
