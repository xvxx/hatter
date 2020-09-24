use {
    crate::{Args, Builtin, Result, Value},
    std::{collections::HashMap, rc::Rc},
};

pub fn builtins() -> HashMap<String, Rc<Builtin>> {
    let mut map: HashMap<String, Rc<Builtin>> = HashMap::new();

    fn eq(args: Args) -> Result<Value> {
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
        .ok()
    }
    fn neq(args: Args) -> Result<Value> {
        Value::Bool(match eq(args)? {
            Value::Bool(b) => !b,
            _ => false,
        })
        .ok()
    }
    fn not(args: Args) -> Result<Value> {
        if let Some(val) = args.get(0) {
            match val {
                Value::None | Value::Bool(false) => Value::Bool(true),
                _ => Value::Bool(false),
            }
        } else {
            Value::None
        }
        .ok()
    }
    fn concat(args: Args) -> Result<Value> {
        let mut sum = String::new();
        for arg in args {
            sum.push_str(&arg.to_string());
        }
        Ok(sum.into())
    }
    fn when(args: Args) -> Result<Value> {
        let fst = args.need(0)?;
        if matches!(fst, Value::None | Value::Bool(false)) {
            Ok(Value::None)
        } else {
            Ok(args.need(1)?.clone())
        }
    }
    fn range(args: Args) -> Result<Value> {
        let start = args.need_number(0)? as i32;
        let end = args.need_number(1)? as i32;
        Ok((start..end).collect::<Vec<_>>().into())
    }
    fn range_inclusive(args: Args) -> Result<Value> {
        let start = args.need_number(0)? as i32;
        let end = args.need_number(1)? as i32;
        Ok((start..=end).collect::<Vec<_>>().into())
    }
    fn index(args: Args) -> Result<Value> {
        if args.len() != 2 {
            return Value::None.ok();
        }
        let subject = args.need(0)?;
        let verb = args.need(1)?;

        match subject {
            Value::Map(map) => map.get(verb.to_str()).unwrap_or(&Value::None).clone(),
            Value::List(list) => {
                let idx = args.need_number(1)?;
                list.get(idx as usize).unwrap_or(&Value::None).clone()
            }
            Value::Object(o) => o.get(verb.to_str()).unwrap_or(Value::None),
            _ => Value::None,
        }
        .ok()
    }
    fn add(args: Args) -> Result<Value> {
        if let Some(Value::Number(_)) = args.get(0) {
            let mut sum = 0.0;
            let mut iter = args.iter();
            while let Some(Value::Number(x)) = iter.next() {
                sum += x;
            }
            return Value::Number(sum).ok();
        } else if let Some(Value::String(_)) = args.get(0) {
            let mut sum = String::new();
            let mut iter = args.iter();
            while let Some(Value::String(x)) = iter.next() {
                sum += x;
            }
            return Value::String(sum).ok();
        }
        Value::None.ok()
    }
    fn sub(args: Args) -> Result<Value> {
        let a = args.need_number(0)?;
        let b = args.need_number(1)?;
        Value::Number(a - b).ok()
    }
    fn mul(args: Args) -> Result<Value> {
        let a = args.need_number(0)?;
        let b = args.need_number(1)?;
        Value::Number(a * b).ok()
    }
    fn div(args: Args) -> Result<Value> {
        let a = args.need_number(0)?;
        let b = args.need_number(1)?;
        Value::Number(a / b).ok()
    }
    fn r#mod(args: Args) -> Result<Value> {
        let a = args.need_number(0)?;
        let b = args.need_number(1)?;
        Value::Number(a % b).ok()
    }
    fn print(mut args: Args) -> Result<Value> {
        while !args.is_empty() {
            let arg = args.remove(0);
            if args.is_empty() {
                args.env.print(format!("{}", arg));
            } else {
                args.env.print(format!("{} ", arg));
            }
        }
        Value::None.ok()
    }
    fn gt(args: Args) -> Result<Value> {
        Value::Bool(args.need_number(0)? > args.need_number(1)?).ok()
    }
    fn gte(args: Args) -> Result<Value> {
        Value::Bool(args.need_number(0)? >= args.need_number(1)?).ok()
    }
    fn lt(args: Args) -> Result<Value> {
        Value::Bool(args.need_number(0)? < args.need_number(1)?).ok()
    }
    fn lte(args: Args) -> Result<Value> {
        Value::Bool(args.need_number(0)? <= args.need_number(1)?).ok()
    }
    fn to_uppercase(args: Args) -> Result<Value> {
        Value::String(args.need_string(0)?.to_uppercase()).ok()
    }
    fn to_lowercase(args: Args) -> Result<Value> {
        Value::String(args.need_string(0)?.to_lowercase()).ok()
    }
    fn replace(args: Args) -> Result<Value> {
        let s = args.need_string(0)?;
        let search = args.need_string(1)?;
        let replace = args.need_string(2)?;
        Value::String(s.replace(search, replace)).ok()
    }
    fn len(args: Args) -> Result<Value> {
        match args.need(0)? {
            Value::List(list) => list.len().into(),
            Value::Map(map) => map.len().into(),
            Value::String(s) => s.len().into(),
            _ => Value::Number(0.0),
        }
        .ok()
    }
    fn empty_(args: Args) -> Result<Value> {
        Value::Bool(len(args)?.to_f64() == 0.0).ok()
    }
    fn r#type(args: Args) -> Result<Value> {
        Value::String(args.need(0)?.typename().into()).ok()
    }

    macro_rules! builtin {
        ($name:expr => $fn:expr) => {
            map.insert($name.to_string(), rc!($fn));
        };
    }

    builtin!("type" => r#type);
    builtin!("when" => when);
    builtin!("==" => eq);
    builtin!("!=" => neq);
    builtin!("not" => not);
    builtin!("concat" => concat);
    builtin!("index" => index);
    builtin!("." => index);
    builtin!(".." => range);
    builtin!("..=" => range_inclusive);
    builtin!("+" => add);
    builtin!("-" => sub);
    builtin!("*" => mul);
    builtin!("/" => div);
    builtin!("%" => r#mod);
    builtin!(">" => gt);
    builtin!(">=" => gte);
    builtin!("<" => lt);
    builtin!("<=" => lte);
    builtin!("print" => print);
    builtin!("to-uppercase" => to_uppercase);
    builtin!("to-lowercase" => to_lowercase);
    builtin!("replace" => replace);
    builtin!("len" => len);
    builtin!("empty?" => empty_);

    map
}
