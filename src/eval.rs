use {
    crate::{Expr, Result, Tag, AST},
    std::collections::HashMap,
};

struct Env<'p> {
    out: String,
    env: HashMap<String, Expr>,
    parent: Option<&'p Env<'p>>,
}

impl<'p> Env<'p> {
    fn new() -> Env<'p> {
        Env {
            out: String::new(),
            env: HashMap::new(),
            parent: None,
        }
    }

    fn putc(&mut self, c: char) {
        self.out.push(c);
    }

    fn print<S: AsRef<str>>(&mut self, it: S) {
        self.out.push_str(it.as_ref());
    }

    fn lookup(&self, name: &str) -> Option<&Expr> {
        if let Some(v) = self.env.get(name) {
            Some(v)
        } else if let Some(p) = self.parent {
            p.lookup(name)
        } else {
            None
        }
    }
}

pub fn eval(ast: AST) -> Result<String> {
    let mut env = Env::new();
    let mut auto_html = false;

    // If the first tag is <head>, add doctype and <html>
    if let Some(Expr::Tag(t)) = ast.exprs.get(0) {
        if t.tag == "head" {
            auto_html = true;
            env.print("<!DOCTYPE html><html>");
        }
    }

    for expr in &ast.exprs {
        eval_expr(&mut env, expr)?;
    }

    // tidy up
    if auto_html {
        env.print("</html>");
    }

    Ok(env.out)
}

fn eval_exprs(env: &mut Env, exprs: &[Expr]) -> Result<()> {
    for expr in exprs {
        eval_expr(env, expr)?;
    }
    Ok(())
}

fn eval_expr(env: &mut Env, expr: &Expr) -> Result<()> {
    use Expr::*;

    match expr {
        Block(exprs) => {
            for expr in exprs {
                eval_expr(env, expr)?;
            }
        }
        Tag(tag) => eval_tag(env, tag)?,
        If => unimplemented!(),
        For => unimplemented!(),
        None => unimplemented!(),
        Bool(b) => env.print(b.to_string()),
        Number(num) => env.print(num.to_string()),
        String(s) => env.print(s),
        Word(word) => {
            if let Some(val) = env.lookup(word) {
                let s = val.to_string();
                env.print(s);
            } else {
                env.print(format!("<undefined word: {}>", word));
            }
        }
        Call(_name, _args) => {
            env.print("<function call>");
        }
    }
    Ok(())
}

fn eval_tag(env: &mut Env, tag: &Tag) -> Result<()> {
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
        eval_exprs(env, &tag.contents)?;
    }

    env.putc('<');
    env.putc('/');
    env.print(&tag.tag);
    env.putc('>');

    Ok(())
}
