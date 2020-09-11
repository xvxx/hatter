use {
    crate::{Builtin, Value, VM},
    std::collections::HashMap,
};

pub fn builtins() -> HashMap<String, Builtin> {
    let mut map: HashMap<String, Builtin> = HashMap::new();

    fn eq(_: &mut VM, args: &[Value]) -> Value {
        if let Some(val) = args.get(0) {
            match val {
                Value::None => match args.get(1) {
                    Some(Value::None) => true,
                    _ => false,
                },
                Value::Bool(b1) => match args.get(1) {
                    Some(Value::Bool(b2)) => b1 == b2,
                    _ => false,
                },
                Value::Number(n1) => match args.get(1) {
                    Some(Value::Number(n2)) => n1 == n2,
                    _ => false,
                },
                Value::String(s1) => match args.get(1) {
                    Some(Value::String(s2)) => s1 == s2,
                    _ => false,
                },
                _ => false,
            }
            .into()
        } else {
            Value::None
        }
    }
    fn neq(vm: &mut VM, args: &[Value]) -> Value {
        match eq(vm, args) {
            Value::Bool(b) => !b,
            _ => false,
        }
        .into()
    }
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
        if let Some(Value::Number(_)) = args.get(0) {
            let mut sum = 0.0;
            let mut iter = args.iter();
            while let Some(Value::Number(x)) = iter.next() {
                sum += x;
            }
            return Value::Number(sum);
        } else if let Some(Value::String(_)) = args.get(0) {
            let mut sum = String::new();
            let mut iter = args.iter();
            while let Some(Value::String(x)) = iter.next() {
                sum += x;
            }
            return Value::String(sum);
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

    macro_rules! builtin {
        ($name:expr => $fn:expr) => {
            map.insert($name.to_string(), $fn);
        };
    }

    builtin!("eq" => eq);
    builtin!("neq" => neq);
    builtin!("not" => not);
    builtin!("add" => add);
    builtin!("sub" => sub);
    builtin!("mul" => mul);
    builtin!("div" => div);
    builtin!("gt" => gt);
    builtin!("gte" => gte);
    builtin!("lt" => lt);
    builtin!("lte" => lte);
    builtin!("print" => print);
    builtin!("to-uppercase" => to_uppercase);
    builtin!("to-lowercase" => to_lowercase);

    map
}
