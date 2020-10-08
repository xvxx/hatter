#![deny(unused_must_use)]
#![deny(unreachable_patterns)]

#[macro_use]
mod macros;
#[macro_use]
mod error;
mod compile;
mod eval;
mod interner;
pub mod prelude;
mod symbol;
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
        value::{self, Fn, NativeFn, Object, SpecialFn, Value},
    },
    interner::Interner,
    symbol::Symbol,
    template::Template,
};

pub type Result<T> = std::result::Result<T, Error>;
