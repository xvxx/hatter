mod error;

pub use error::{print_error, Error};

pub struct AST {}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Token {
    None,
}

pub fn scan(_src: &str) -> Result<Vec<Token>> {
    Ok(vec![])
}

pub fn parse(_tokens: Vec<Token>) -> Result<AST> {
    Ok(AST {})
}

pub fn compile(_ast: AST) -> Result<String> {
    Ok(String::new())
}

pub fn to_html(source: &str) -> Result<String> {
    Ok(source.to_string())
}
