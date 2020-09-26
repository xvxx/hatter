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

macro_rules! cell {
    ($ex:expr) => {
        Rc::new(RefCell::new($ex))
    };
}

#[macro_use]
mod error;
mod compile;
mod eval;
mod template;

#[cfg(feature = "repl")]
pub mod repl;

pub use {
    compile::{
        compile,
        lexer::scan,
        parser::parse,
        stmt::Stmt,
        syntax::{Syntax, SyntaxTrait},
        tag::Tag,
        token::Token,
    },
    error::{print_error, Error, ErrorKind},
    eval::{
        args::Args,
        env::{eval, render, Env, Jump, Scope},
        natives::{natives, specials},
        value::{FnType, NativeFn, Object, SpecialFn, Value},
    },
    template::Template,
};

pub type Result<T> = std::result::Result<T, Error>;
