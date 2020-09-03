#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
mod error;
mod parser;
mod scanner;
mod token;

pub use {
    error::{print_error, Error},
    parser::parse,
    scanner::scan,
    token::{Token, TokenKind, TokenStream},
};

#[derive(Debug)]
pub struct AST {}

impl AST {
    pub fn new() -> AST {
        AST {}
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn compile(_ast: AST) -> Result<String> {
    Ok(String::new())
}

pub fn to_html(source: &str) -> Result<String> {
    Ok(source.to_string())
}
