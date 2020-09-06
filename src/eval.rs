use {
    crate::{Expr, Result, Tag, AST},
    std::collections::HashMap,
};

type HatFn = fn(&mut Env, &[Value]) -> Value;

fn builtins() -> HashMap<String, Value> {
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

#[derive(Clone)]
enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Fn(HatFn),
    List(Vec<Value>),
    Map(HashMap<Value, Value>),
}

impl Value {
    fn typename(&self) -> &str {
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

struct Env<'p> {
    out: String,
    env: HashMap<String, Value>,
    builtins: HashMap<String, Value>,
    parent: Option<&'p Env<'p>>,
}

impl<'p> Env<'p> {
    fn root() -> Env<'p> {
        let mut new = Env::new();
        new.builtins = builtins();
        new
    }

    fn new() -> Env<'p> {
        Env {
            out: String::new(),
            env: HashMap::new(),
            builtins: HashMap::new(),
            parent: None,
        }
    }

    fn from(env: &'p Env) -> Env<'p> {
        let mut new = Env::new();
        new.parent = Some(env);
        new
    }

    fn putc(&mut self, c: char) {
        self.out.push(c);
    }

    fn print<S: AsRef<str>>(&mut self, it: S) {
        self.out.push_str(it.as_ref());
    }

    fn lookup(&self, name: &str) -> Option<&Value> {
        if let Some(v) = self.env.get(name) {
            Some(v)
        } else if let Some(p) = self.parent {
            p.lookup(name)
        } else if let Some(b) = self.builtins.get(name) {
            Some(b)
        } else {
            None
        }
    }
}

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

    Ok(env.out)
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
