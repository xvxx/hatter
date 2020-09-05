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
    Bool(bool),
    String(String),
    Word(String),
    Number(f64),
    Call(String, Vec<Expr>),
}

impl Expr {
    pub fn literal(&self) -> &str {
        match self {
            Expr::String(s) | Expr::Word(s) => &s,
            _ => "",
        }
    }

    pub fn to_string(&self) -> String {
        self.literal().to_string()
    }
}
