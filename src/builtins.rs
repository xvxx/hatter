use {
    crate::{Builtin, Value, VM},
    std::{collections::HashMap, rc::Rc},
};

pub fn builtins() -> HashMap<String, Rc<Builtin>> {
    let mut map: HashMap<String, Rc<Builtin>> = HashMap::new();

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
    fn concat(_: &mut VM, args: &[Value]) -> Value {
        let mut sum = String::new();
        for arg in args {
            sum.push_str(&arg.to_string());
        }
        return Value::String(sum);
    }
    fn when(_: &mut VM, args: &[Value]) -> Value {
        if matches!(&args[0], Value::None | Value::Bool(false)) {
            return Value::None;
        } else {
            return args[1].clone();
        }
    }
    fn index(_: &mut VM, args: &[Value]) -> Value {
        if args.len() != 2 {
            return Value::None;
        }
        let subject = &args[0];
        let verb = &args[1];

        match subject {
            Value::Map(map) => map.get(verb.to_str()).unwrap_or(&Value::None).clone(),
            Value::List(list) => {
                if let Value::Number(n) = verb {
                    list.get(*n as usize).unwrap_or(&Value::None).clone()
                } else {
                    Value::None
                }
            }
            Value::Object(o) => o.get(verb.to_str()).unwrap_or(Value::None),
            _ => Value::None,
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
        let len = args.len();
        for (i, arg) in args.iter().enumerate() {
            if i == len - 1 {
                println!("{}", arg);
            } else {
                print!("{} ", arg);
            }
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
    fn len(_: &mut VM, args: &[Value]) -> Value {
        match args.get(0) {
            Some(Value::List(list)) => list.len().into(),
            Some(Value::Map(map)) => map.len().into(),
            Some(Value::String(s)) => s.len().into(),
            _ => 0.into(),
        }
    }
    fn empty_(vm: &mut VM, args: &[Value]) -> Value {
        if let Value::Number(n) = len(vm, args) {
            n == 0.0
        } else {
            false
        }
        .into()
    }
    fn r#type(_: &mut VM, args: &[Value]) -> Value {
        if args.is_empty() {
            return Value::None;
        }
        args[0].typename().into()
    }

    macro_rules! builtin {
        ($name:expr => $fn:expr) => {
            map.insert($name.to_string(), rc!($fn));
        };
    }

    builtin!("type" => r#type);
    builtin!("when" => when);
    builtin!("eq" => eq);
    builtin!("neq" => neq);
    builtin!("not" => not);
    builtin!("concat" => concat);
    builtin!("index" => index);
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
    builtin!("len" => len);
    builtin!("empty?" => empty_);

    map
}
