#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
mod error;
mod scanner;
mod token;

pub use {
    error::{print_error, Error},
    scanner::scan,
    token::{Token, TokenKind, TokenStream},
};

#[derive(Debug)]
pub struct AST {}

pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(_tokens: TokenStream) -> Result<AST> {
    Ok(AST {})
}

pub fn compile(_ast: AST) -> Result<String> {
    Ok(String::new())
}

pub fn to_html(source: &str) -> Result<String> {
    Ok(source.to_string())
}
