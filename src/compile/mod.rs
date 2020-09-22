pub mod lexer;
pub mod parser;
pub mod stmt;
pub mod syntax;
pub mod tag;
pub mod token;

/// Compile source code into a Vec of Stmt, aka the AST.
pub fn compile<S: AsRef<str>>(s: S) -> crate::Result<Vec<stmt::Stmt>> {
    use {lexer::scan, parser::parse};
    scan(s.as_ref()).and_then(|t| parse(&t))
}
