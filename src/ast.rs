use crate::Stmt;

#[derive(Debug)]
pub struct AST {
    pub stmts: Vec<Stmt>,
}

impl AST {
    pub fn new() -> AST {
        AST { stmts: vec![] }
    }
}
