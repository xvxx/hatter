#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
mod error;
mod ast;
mod parser;
mod scanner;
mod stmt;
mod tag;
mod token;

pub use {
    ast::AST,
    error::{print_error, Error},
    parser::parse,
    scanner::scan,
    stmt::Stmt,
    tag::Tag,
    token::{Token, TokenKind, TokenStream},
};

pub type Result<T> = std::result::Result<T, Error>;

pub fn compile(_ast: AST) -> Result<String> {
    Ok(String::new())
}

pub fn to_html(source: &str) -> Result<String> {
    Ok(source.to_string())
}
