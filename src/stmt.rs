use crate::Tag;

#[derive(Debug)]
pub enum Stmt {
    None,
    Text,
    If,
    For,
    Expr,
    Tag(Tag),
}
