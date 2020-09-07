use {
    crate::{Builtin, Value, VM},
    std::collections::HashMap,
};

pub fn builtins() -> HashMap<String, Builtin> {
    let mut map: HashMap<String, Builtin> = HashMap::new();

    fn not(_: &mut VM, args: &[Value]) -> Value {
        if let Some(val) = args.get(0) {
            match val {
                Value::None | Value::Bool(false) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        } else {
            Value::None
        }
    }
    fn add(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a + b);
            }
        }
        Value::None
    }
    fn sub(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a - b);
            }
        }
        Value::None
    }
    fn mul(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a * b);
            }
        }
        Value::None
    }
    fn div(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Number(a / b);
            }
        }
        Value::None
    }
    fn print(_: &mut VM, args: &[Value]) -> Value {
        for arg in args {
            println!("{}", arg);
        }
        Value::None
    }
    fn gt(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Bool(a > b);
            }
        }
        Value::None
    }
    fn gte(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Bool(a >= b);
            }
        }
        Value::None
    }
    fn lt(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Bool(a < b);
            }
        }
        Value::None
    }
    fn lte(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::Number(a)) = args.get(0) {
            if let Some(Value::Number(b)) = args.get(1) {
                return Value::Bool(a <= b);
            }
        }
        Value::None
    }
    fn to_uppercase(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::String(s)) = args.get(0) {
            Value::String(s.to_uppercase())
        } else {
            Value::String("Expected String".to_string())
        }
    }
    fn to_lowercase(_: &mut VM, args: &[Value]) -> Value {
        if let Some(Value::String(s)) = args.get(0) {
            Value::String(s.to_lowercase())
        } else {
            Value::String("Expected String".to_string())
        }
    }

    map.insert("not".to_string(), not);
    map.insert("add".to_string(), add);
    map.insert("sub".to_string(), sub);
    map.insert("mul".to_string(), mul);
    map.insert("div".to_string(), div);
    map.insert("gt".to_string(), gt);
    map.insert("gte".to_string(), gte);
    map.insert("lt".to_string(), lt);
    map.insert("lte".to_string(), lte);
    map.insert("print".to_string(), print);
    map.insert("to-uppercase".to_string(), to_uppercase);
    map.insert("to-lowercase".to_string(), to_lowercase);

    map
}
