//! Hatter comes with a few built-in functions to help make your life
//! easier. All of them are defined in this file, and each comes in
//! one of two flavors of function: `Native` or `Special`.
//!
//! `Native` is a regular function that takes an `Args` struct and
//! returns a `Result<Value>`, but `Special` is more like a macro
//! (or fexpr): its arguments are not evaluated, but instead passed to
//! the function as syntax. The scope of the caller is also passed to
//! the special function. This means the function can then decide how,
//! when, and if to evaluate arguments or not. We use this to
//! implement short circuiting in `&&` and `||`, but it could have
//! other applications, too.

use {
    crate::{Args, Env, NativeFn, Result, SpecialFn, Stmt, Value},
    std::{collections::HashMap, rc::Rc},
};

/// Return the builtin Special functions that come with Hatter.
/// Ideally we build this at compile time, at least in release mode,
/// instead of creating them all at runtime.
pub(crate) fn specials() -> HashMap<String, Rc<SpecialFn>> {
    let mut map: HashMap<String, Rc<SpecialFn>> = HashMap::new();

    map.insert("&&".into(), rc!(and));
    map.insert("||".into(), rc!(or));

    map
}

/// Return the builtin Native functions that come with Hatter.
pub(crate) fn natives() -> HashMap<String, Rc<NativeFn>> {
    let mut map: HashMap<String, Rc<NativeFn>> = HashMap::new();

    macro_rules! native {
        ($name:expr => $fn:expr) => {
            map.insert($name.to_string(), rc!($fn));
        };
    }

    native!("type" => r#type);
    native!("when" => when);
    native!("==" => eq);
    native!("!=" => neq);
    native!("not" => not);
    native!("!" => not);
    native!("concat" => concat);
    native!("index" => index);
    native!("<<" => push);
    native!("." => index);
    native!(".." => range);
    native!("..=" => range_inclusive);
    native!("+" => add);
    native!("-" => sub);
    native!("*" => mul);
    native!("/" => div);
    native!("%" => r#mod);
    native!(">" => gt);
    native!(">=" => gte);
    native!("<" => lt);
    native!("<=" => lte);
    native!("print" => print);
    native!("to-uppercase" => to_uppercase);
    native!("to-lowercase" => to_lowercase);
    native!("replace" => replace);
    native!("len" => len);
    native!("empty?" => empty_);

    map
}

//////////////////////////////////////////////////////////////////////
// Internal Functions

/// Combine several Values into a String.
/// Used internally by String interpolation.
/// `concat("hi", "-", 23) #=> "hi-23"`
fn concat(args: Args) -> Result<Value> {
    let mut sum = String::new();
    for arg in args {
        sum.push_str(&arg.to_string());
    }
    Ok(sum.into())
}

/// Return a Value if a condition is true.
/// Used internally by tag attributes.
/// `when(true, "yep")   #=> "yep"`
/// `when(false, "nope") #=> None`
fn when(args: Args) -> Result<Value> {
    let fst = args.need(0)?;
    if matches!(fst, Value::None | Value::Bool(false)) {
        Ok(Value::None)
    } else {
        Ok(args.need(1)?.clone())
    }
}

/// Get the String name of a Value's type.
/// `type('heyo')  #=> "String"`
/// `type(123)     #=> "Number"`
fn r#type(args: Args) -> Result<Value> {
    Value::String(args.need(0)?.typename().into()).ok()
}

//////////////////////////////////////////////////////////////////////
// Boolean Operators

/// Special: Short-circuiting `&&` operator.
fn and(env: &mut Env, args: &[Stmt]) -> Result<Value> {
    if args.len() != 2 {
        return error!("Expected 2 args, got {}", 2);
    }
    Ok((env.eval(&args[0])?.to_bool() && env.eval(&args[1])?.to_bool()).into())
}

/// Special: Short-circuiting `||` operator.
fn or(env: &mut Env, args: &[Stmt]) -> Result<Value> {
    if args.len() != 2 {
        return error!("Expected 2 args, got {}", 2);
    }
    Ok((env.eval(&args[0])?.to_bool() || env.eval(&args[1])?.to_bool()).into())
}

/// `==` operator
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

/// `!=` operator
fn neq(args: Args) -> Result<Value> {
    Value::Bool(match eq(args)? {
        Value::Bool(b) => !b,
        _ => false,
    })
    .ok()
}

/// `!` operator
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

/// `>` operator
fn gt(args: Args) -> Result<Value> {
    Value::Bool(args.need_number(0)? > args.need_number(1)?).ok()
}

/// `>=` operator
fn gte(args: Args) -> Result<Value> {
    Value::Bool(args.need_number(0)? >= args.need_number(1)?).ok()
}

/// `<` operator
fn lt(args: Args) -> Result<Value> {
    Value::Bool(args.need_number(0)? < args.need_number(1)?).ok()
}

/// `<=` operator
fn lte(args: Args) -> Result<Value> {
    Value::Bool(args.need_number(0)? <= args.need_number(1)?).ok()
}

//////////////////////////////////////////////////////////////////////
// Math Functions

/// `+` operator
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
        return Value::String(sum.into()).ok();
    }
    Value::None.ok()
}

/// `-` operator
fn sub(args: Args) -> Result<Value> {
    let a = args.need_number(0)?;
    let b = args.need_number(1)?;
    Value::Number(a - b).ok()
}

/// `*` operator
fn mul(args: Args) -> Result<Value> {
    let a = args.need_number(0)?;
    let b = args.need_number(1)?;
    Value::Number(a * b).ok()
}

/// `/` operator
fn div(args: Args) -> Result<Value> {
    let a = args.need_number(0)?;
    let b = args.need_number(1)?;
    Value::Number(a / b).ok()
}

/// `%` operator
fn r#mod(args: Args) -> Result<Value> {
    let a = args.need_number(0)?;
    let b = args.need_number(1)?;
    Value::Number(a % b).ok()
}

//////////////////////////////////////////////////////////////////////
// String Functions

/// Rust's String::to_uppercase()
fn to_uppercase(args: Args) -> Result<Value> {
    Value::String(args.need_string(0)?.to_uppercase().into()).ok()
}

/// Rust's String::to_lowercase()
fn to_lowercase(args: Args) -> Result<Value> {
    Value::String(args.need_string(0)?.to_lowercase().into()).ok()
}

/// `replace("Mr Rogers", "Ro", "Dod") #=> "Mr Dodgers"`
fn replace(args: Args) -> Result<Value> {
    let s = args.need_string(0)?;
    let search = args.need_string(1)?;
    let replace = args.need_string(2)?;
    Value::String(s.replace(search, replace).into()).ok()
}

//////////////////////////////////////////////////////////////////////
// Container (Map, List, Object) Functions

/// `.` operator and `[]` operator
/// Look up a `List` item by number or a `Map` item by key.
/// Returns None or the value.
fn index(args: Args) -> Result<Value> {
    if args.len() != 2 {
        return Value::None.ok();
    }
    let subject = args.need(0)?;

    match subject {
        Value::Map(map) => map
            .borrow()
            .get(&args.need_string(1)?.into())
            .unwrap_or(&Value::None)
            .clone(),
        Value::List(list) => {
            let idx = args.need_number(1)?;
            list.borrow()
                .get(idx as usize)
                .unwrap_or(&Value::None)
                .clone()
        }
        Value::Object(o) => o.get(args.need_string(1)?).unwrap_or(Value::None),
        _ => Value::None,
    }
    .ok()
}

/// `len([])            #=> 0`
/// `len([5])           #=> 1`
/// `len('hi')          #=> 2`
/// `len({name:'Ra'})   #=> 1`
fn len(args: Args) -> Result<Value> {
    match args.need(0)? {
        Value::List(list) => list.borrow().len().into(),
        Value::Map(map) => map.borrow().len().into(),
        Value::String(s) => s.len().into(),
        _ => Value::Number(0.0),
    }
    .ok()
}

/// `empty?([])  #=> true`
/// `empty?([1]) #=> false`
fn empty_(args: Args) -> Result<Value> {
    Value::Bool(len(args)?.to_f64() == 0.0).ok()
}

//////////////////////////////////////////////////////////////////////
// List Functions

/// Add a Value to a `List`. Modifies the `List`.
fn push(args: Args) -> Result<Value> {
    if let Value::List(list) = args.need(0)? {
        list.borrow_mut().push(args.need(1)?);
    }
    Value::None.ok()
}

//////////////////////////////////////////////////////////////////////
// Range Functions

/// `..` operator
/// `1..5  #=> [1,2,3,4]`
fn range(args: Args) -> Result<Value> {
    let start = args.need_number(0)? as i32;
    let end = args.need_number(1)? as i32;
    Ok((start..end).collect::<Vec<_>>().into())
}

/// `..=` operator
/// `1..=5  #=> [1,2,3,4,5]`
fn range_inclusive(args: Args) -> Result<Value> {
    let start = args.need_number(0)? as i32;
    let end = args.need_number(1)? as i32;
    Ok((start..=end).collect::<Vec<_>>().into())
}

//////////////////////////////////////////////////////////////////////
// I/O Functions

/// Print one or more Values, with newline.
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
