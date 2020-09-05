use crate::{Expr,Tag};

#[derive(Debug)]
pub enum Stmt {
    None,
    Block(Vec<Stmt>),
    Text,
    If,
    For,
    Expr(Expr),
    Tag(Tag),
    Close(String),
}
