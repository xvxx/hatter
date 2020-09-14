#![deny(unused_must_use)]
#![deny(unreachable_patterns)]

macro_rules! bx {
    ($ex:expr) => {
        Box::new($ex)
    };
}

#[macro_use]
mod error;
mod builtins;
mod compiler;
mod expr;
mod lexer;
mod parser;
mod syntax;
mod template;
mod token;
mod value;
pub mod vm;

#[cfg(feature = "repl")]
pub mod repl;

pub use {
    builtins::builtins,
    compiler::{compile, Code},
    error::{print_error, Error},
    expr::{Expr, Tag},
    lexer::scan,
    parser::parse,
    syntax::Syntax,
    template::Template,
    token::Token,
    value::{Builtin, Value},
    vm::{Scope, VM},
};

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_html(s: &str) -> Result<String> {
    scan(s)
        .and_then(|t| parse(&t))
        .and_then(|a| compile(&a))
        .and_then(|c| vm::run(&c))
}
