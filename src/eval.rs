use {
    crate::{Env, Expr, HatterFn, Result, Tag, Value, AST},
    std::collections::HashMap,
};

pub fn eval(ast: AST) -> Result<String> {
    let mut env = Env::root();
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
    match expr {
        Expr::Tag(t) => print_tag(env, t)?,
        _ => match eval_expr(env, expr)? {
            Value::String(s) => env.print(&s),
            Value::Number(n) => env.print(&n.to_string()),
            Value::Bool(b) => env.print(&b.to_string()),
            _ => unimplemented!(),
        },
    }
    Ok(())
}

fn eval_expr(env: &mut Env, expr: &Expr) -> Result<Value> {
    use Expr::*;

    Ok(match expr {
        None | If | For | Tag(..) => unimplemented!(),
        Bool(b) => Value::Bool(*b),
        Number(n) => Value::Number(*n),
        String(n) => Value::String(n.clone()),
        Word(word) => {
            if let Some(val) = env.lookup(word) {
                val.clone()
            } else {
                eprintln!("<undefined word: {}>", word); // TODO
                Value::String(word.clone())
            }
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

fn print_tag(env: &mut Env, tag: &Tag) -> Result<()> {
    env.putc('<');
    let is_form = tag.tag == "form";
    env.print(&tag.tag);

    if !tag.classes.is_empty() {
        env.print(" class='");
        let len = tag.classes.len();
        for (i, class) in tag.classes.iter().enumerate() {
            env.print(class);
            if i < len - 1 {
                env.putc(' ');
            }
        }
        env.print("'");
    }

    for (name, val) in &tag.attrs {
        if is_form && (name == "GET" || name == "POST") {
            env.print(&format!(" method='{}' action='{}'", name, val));
            continue;
        }
        env.putc(' ');
        env.print(&name);
        env.putc('=');
        env.putc('\'');
        env.print(&val);
        env.putc('\'');
    }

    if tag.tag == "a" && !tag.attrs.contains_key("href") {
        env.print(" href='#'");
    }

    if tag.is_closed() {
        env.putc('/');
        env.putc('>');
        return Ok(());
    } else {
        env.putc('>');
    }

    if !tag.contents.is_empty() {
        print_exprs(env, &tag.contents)?;
    }

    env.putc('<');
    env.putc('/');
    env.print(&tag.tag);
    env.putc('>');

    Ok(())
}
