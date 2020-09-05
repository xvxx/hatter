use crate::{Expr, Result, Stmt, Tag};

/// Turn AST into HTML.
pub fn emit(stmts: Vec<Stmt>) -> Result<String> {
    let mut out = String::new();
    let mut auto_html = false;

    // If the first tag is <head>, emit <html>
    if let Some(Stmt::Tag(t)) = stmts.get(0) {
        if t.tag == "head" {
            auto_html = true;
            out.push_str("<!DOCTYPE html><html>");
        }
    }

    for stmt in stmts {
        out.push_str(&match stmt {
            Stmt::Tag(t) => tag(t)?,
            Stmt::Expr(e) => expr(e)?,
            _ => unimplemented!("dog"),
        });
    }

    if auto_html {
        out.push_str("</html>");
    }

    Ok(out)
}

/// Just print as literals for now.
pub fn expr(expr: Expr) -> Result<String> {
    match expr {
        Expr::String(s) | Expr::Word(s) => Ok(format!("{} ", s)),
    }
}

pub fn tag(tag: Tag) -> Result<String> {
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
        return Ok(out);
    } else {
        out.push('>');
    }

    if !tag.contents.is_empty() {
        out.push_str(&emit(tag.contents)?);
    }

    out.push('<');
    out.push('/');
    out.push_str(&tag.tag);
    out.push('>');

    Ok(out)
}
