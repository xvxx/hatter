use crate::{Expr, Result, Stmt, Tag};

/// Turn AST into HTML.
pub fn emit(stmts: Vec<Stmt>) -> Result<String> {
    let mut out = String::new();

    for stmt in stmts {
        out.push_str(&match stmt {
            Stmt::Tag(t) => tag(t)?,
            Stmt::Expr(e) => expr(e)?,
            _ => unimplemented!("dog"),
        });
    }

    Ok(out)
}

/// Just print as literals for now.
pub fn expr(expr: Expr) -> Result<String> {
    match expr {
        Expr::String(s) => Ok(s),
        _ => unimplemented!(),
    }
}

pub fn tag(tag: Tag) -> Result<String> {
    let mut out = String::new();
    out.push('<');
    out.push_str(&tag.tag);

    if !tag.classes.is_empty() {
        out.push_str(" class='");
        for class in &tag.classes {
            out.push_str(class);
            out.push(' ');
        }
        out.push_str("'");
    }

    for (name, val) in &tag.attrs {
        out.push(' ');
        out.push_str(&name);
        out.push('=');
        out.push('\'');
        out.push_str(&val);
        out.push('\'');
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
