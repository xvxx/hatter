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
mod compile;
mod env;
mod template;
mod value;

#[cfg(feature = "repl")]
pub mod repl;

pub use {
    builtins::builtins,
    compile::{
        compile,
        lexer::scan,
        parser::parse,
        stmt::Stmt,
        syntax::{Syntax, SyntaxTrait},
        tag::Tag,
        token::Token,
    },
    env::{eval, render, Env, Jump, Scope},
    error::{print_error, Error, ErrorKind},
    template::Template,
    value::{Builtin, Object, Value},
};

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_html(s: &str) -> Result<String> {
    compile(s).and_then(|p| render(&p))
}
