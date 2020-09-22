#![deny(unused_must_use)]
#![deny(unreachable_patterns)]

macro_rules! bx {
    ($ex:expr) => {
        Box::new($ex)
    };
}

macro_rules! rc {
    ($ex:expr) => {
        Rc::new($ex)
    };
}

#[macro_use]
mod error;
mod builtins;
mod env;
mod lexer;
mod parser;
mod stmt;
mod syntax;
mod template;
mod token;
mod value;

#[cfg(feature = "repl")]
pub mod repl;

pub use {
    builtins::builtins,
    env::{eval, render, Env, Jump, Scope},
    error::{print_error, Error, ErrorKind},
    lexer::scan,
    parser::parse,
    stmt::{Stmt, Tag},
    syntax::{Syntax, SyntaxTrait},
    template::Template,
    token::Token,
    value::{Builtin, Object, Value},
};

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_html(s: &str) -> Result<String> {
    scan(s).and_then(|t| parse(&t)).and_then(|p| render(&p))
}
