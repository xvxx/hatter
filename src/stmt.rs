use crate::Tag;

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

#[derive(Debug)]
pub enum Expr {
    String(String),
    Word(String),
}
