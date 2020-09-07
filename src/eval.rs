use {
    crate::{Env, Expr, HatterFn, Result, Tag, Value, AST},
    std::collections::HashMap,
};

trait JoinValues {
    fn join<S: AsRef<str>>(&self, joiner: S) -> Value;
}

impl JoinValues for Vec<Value> {
    fn join<S: AsRef<str>>(&self, joiner: S) -> Value {
        Value::String(
            self.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(joiner.as_ref()),
        )
    }
}

pub fn eval(ast: AST) -> Result<String> {
    let mut env = Env::root();
    env.set("abc", Value::from(vec!["one", "two", "three"]));
    let mut auto_html = false;

    // If the first tag is <head>, add doctype and <html>
    if let Some(Expr::Tag(t)) = ast.exprs.get(0) {
        if t.tag == "head" {
            auto_html = true;
            env.print("<!DOCTYPE html><html>");
        }
    }

    print_exprs(&mut env, &ast.exprs)?;

    // tidy up
    if auto_html {
        env.print("</html>");
    }

    Ok(env.to_string())
}

fn print_exprs(env: &mut Env, exprs: &[Expr]) -> Result<()> {
    for expr in exprs {
        print_expr(env, expr)?;
    }
    Ok(())
}

fn print_expr(env: &mut Env, expr: &Expr) -> Result<()> {
    match eval_expr(env, expr)? {
        Value::String(s) => env.print(&s),
        Value::Number(n) => env.print(&n.to_string()),
        Value::Bool(b) => env.print(&b.to_string()),
        _ => unimplemented!(),
    }
    Ok(())
}

fn eval_exprs(env: &mut Env, exprs: &[Expr]) -> Result<Value> {
    let mut ret = Value::None;
    for expr in exprs {
        ret = eval_expr(env, expr)?;
        if let Value::Break = ret {
            break;
        }
    }
    Ok(ret)
}

fn eval_expr(env: &mut Env, expr: &Expr) -> Result<Value> {
    use Expr::*;

    Ok(match expr {
        None => unimplemented!(),
        Tag(tag) => eval_tag(env, tag)?,
        Bool(b) => Value::Bool(*b),
        Number(n) => Value::Number(*n),
        String(n) => Value::String(n.clone()),
        Word(word) => {
            if word == "break" {
                return Ok(Value::Break);
            } else if let Some(val) = env.lookup(word) {
                val.clone()
            } else {
                eprintln!("<undefined word: {}>", word); // TODO
                Value::String(word.clone())
            }
        }
        For(key, val, iter, body) => {
            let iter = eval_expr(env, iter)?;
            match iter {
                Value::List(list) => {
                    let mut scope = Env::from(env);
                    let mut ret = vec![];
                    'for_list: for (i, v) in list.iter().enumerate() {
                        scope.set(val, v.clone());
                        if let Some(k) = key {
                            scope.set(k, Value::from(i));
                        }
                        for expr in body {
                            println!("{:?}", expr);
                            match eval_expr(&mut scope, expr)? {
                                Value::Break => {
                                    println!("BREAKING!");
                                    break 'for_list;
                                }
                                r @ _ => {
                                    ret.push(r);
                                },
                            }
                        }
                    }
                    ret.join("\n")
                }
                Value::Map(map) => {
                    let mut scope = Env::from(env);
                    let mut ret = vec![];
                    'for_map: for (k, v) in map {
                        scope.set(val, v.clone());
                        if let Some(var) = key {
                            scope.set(var, Value::from(k));
                        }
                        for expr in body {
                            match eval_expr(&mut scope, expr)? {
                                Value::Break => break 'for_map,
                                r @ _ => ret.push(r),
                            }
                        }
                    }
                    ret.join("\n")
                }
                _ => {
                    println!(">> GOT: {:?}", iter.typename());
                    unimplemented!();
                }
            }
        }
        If(conds) => {
            let mut ret = vec![];
            for (test, body) in conds {
                match eval_expr(env, test)? {
                    Value::Bool(false) | Value::None => {}
                    _ => {
                        for expr in body {
                            ret.push(eval_expr(env, expr)?);
                        }
                        break;
                    }
                }
            }
            ret.join("\n")
        }
        Call(name, args) => {
            let mut evaled_args = vec![];
            for arg in args {
                evaled_args.push(eval_expr(env, arg)?);
            }

            if let Some(val) = env.lookup(name) {
                if let Value::Fn(f) = val {
                    f(env, &evaled_args)
                } else {
                    eprintln!("<expected Fn, got {}>", val.typename()); // TODO
                    Value::String(format!("<expected Fn, got {}>", val.typename()))
                }
            } else {
                eprintln!("<undefined function: {}>", name); // TODO
                Value::String(format!("<undefined function: {}>", name))
            }
        }
    })
}

fn eval_tag(env: &mut Env, tag: &Tag) -> Result<Value> {
    let mut out = String::new();
    out.push('<');
    let is_form = tag.tag == "form";
    out.push_str(&tag.tag);

    if !tag.classes.is_empty() {
        out.push_str(" class='");
        let len = tag.classes.len();
        for (i, class) in tag.classes.iter().enumerate() {
            out.push_str(class);
            if i < len - 1 {
                out.push(' ');
            }
        }
        out.push_str("'");
    }

    for (name, val) in &tag.attrs {
        if is_form && (name == "GET" || name == "POST") {
            out.push_str(&format!(" method='{}' action='{}'", name, val));
            continue;
        }
        out.push(' ');
        out.push_str(&name);
        out.push('=');
        out.push('\'');
        out.push_str(&val);
        out.push('\'');
    }

    if tag.tag == "a" && !tag.attrs.contains_key("href") {
        out.push_str(" href='#'");
    }

    if tag.is_closed() {
        out.push('/');
        out.push('>');
        return Ok(Value::String(out));
    } else {
        out.push('>');
    }

    if !tag.contents.is_empty() {
        for expr in &tag.contents {
            out.push_str(&eval_expr(env, expr)?.to_string());
        }
    }

    out.push('<');
    out.push('/');
    out.push_str(&tag.tag);
    out.push('>');

    Ok(Value::String(out))
}
