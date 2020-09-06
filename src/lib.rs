#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(unused_must_use)]

#[macro_use]
mod error;
mod ast;
mod env;
mod eval;
mod expr;
mod parser;
mod scanner;
mod tag;
mod token;

pub use {
    ast::AST,
    env::Env,
    error::{print_error, Error},
    eval::eval,
    expr::Expr,
    parser::parse,
    scanner::scan,
    tag::Tag,
    token::{Syntax, Token, TokenStream},
};

pub type Result<T> = std::result::Result<T, Error>;

pub fn compile(_ast: AST) -> Result<String> {
    Ok(String::new())
}

pub fn to_html(source: &str) -> Result<String> {
    Ok(source.to_string())
}
