use crate::Expr;

#[derive(Debug)]
pub struct AST {
    pub exprs: Vec<Expr>,
}

impl AST {
    pub fn new() -> AST {
        AST { exprs: vec![] }
    }
}
