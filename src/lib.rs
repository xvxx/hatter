#![allow(dead_code)]
#![allow(unused_imports)]
#![deny(unused_must_use)]
#![deny(unreachable_patterns)]

#[macro_use]
mod error;
mod ast;
mod builtins;
mod compiler;
mod env;
mod expr;
mod parser;
mod scanner;
mod tag;
mod token;
mod value;
pub mod vm;

pub use {
    ast::AST,
    builtins::builtins,
    compiler::{compile, Code},
    env::Env,
    error::{print_error, Error},
    expr::Expr,
    parser::parse,
    scanner::scan,
    tag::Tag,
    token::{Syntax, Token, TokenStream},
    value::{HatterFn, Value},
};

pub type Result<T> = std::result::Result<T, Error>;
