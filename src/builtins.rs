use {
    crate::{Env, Value},
    std::collections::HashMap,
};

pub fn builtins() -> HashMap<String, Value> {
    let map: HashMap<String, Value> = HashMap::new();
    // fn not(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(val) = args.get(0) {
    //         match val {
    //             Value::None | Value::Bool(false) => Value::Bool(true),
    //             _ => Value::Bool(false),
    //         }
    //     } else {
    //         Value::None
    //     }
    // }
    // fn add(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Number(a + b);
    //         }
    //     }
    //     Value::None
    // }
    // fn sub(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Number(a - b);
    //         }
    //     }
    //     Value::None
    // }
    // fn mul(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Number(a * b);
    //         }
    //     }
    //     Value::None
    // }
    // fn div(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Number(a / b);
    //         }
    //     }
    //     Value::None
    // }
    // fn print(env: &mut Env, args: &[Value]) -> Value {
    //     for arg in args {
    //         match arg {
    //             Value::None => env.print("None"),
    //             Value::String(s) => env.print(s),
    //             Value::Number(n) => env.print(n.to_string()),
    //             Value::Bool(b) => env.print(b.to_string()),
    //             _ => unimplemented!(),
    //         }
    //     }
    //     Value::None
    // }
    // fn gt(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Bool(a > b);
    //         }
    //     }
    //     Value::None
    // }
    // fn gte(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Bool(a >= b);
    //         }
    //     }
    //     Value::None
    // }
    // fn lt(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Bool(a < b);
    //         }
    //     }
    //     Value::None
    // }
    // fn lte(_: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::Number(a)) = args.get(0) {
    //         if let Some(Value::Number(b)) = args.get(1) {
    //             return Value::Bool(a <= b);
    //         }
    //     }
    //     Value::None
    // }
    // fn to_uppercase(_env: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::String(s)) = args.get(0) {
    //         Value::String(s.to_uppercase())
    //     } else {
    //         Value::String("Expected String".to_string())
    //     }
    // }
    // fn to_lowercase(_env: &mut Env, args: &[Value]) -> Value {
    //     if let Some(Value::String(s)) = args.get(0) {
    //         Value::String(s.to_lowercase())
    //     } else {
    //         Value::String("Expected String".to_string())
    //     }
    // }

    // map.insert("not".to_string(), Value::Fn(not));
    // map.insert("add".to_string(), Value::Fn(add));
    // map.insert("sub".to_string(), Value::Fn(sub));
    // map.insert("mul".to_string(), Value::Fn(mul));
    // map.insert("div".to_string(), Value::Fn(div));
    // map.insert("gt".to_string(), Value::Fn(gt));
    // map.insert("gte".to_string(), Value::Fn(gte));
    // map.insert("lt".to_string(), Value::Fn(lt));
    // map.insert("lte".to_string(), Value::Fn(lte));
    // map.insert("print".to_string(), Value::Fn(print));
    // map.insert("to-uppercase".to_string(), Value::Fn(to_uppercase));
    // map.insert("to-lowercase".to_string(), Value::Fn(to_lowercase));
    map
}
