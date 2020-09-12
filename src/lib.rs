#![deny(unused_must_use)]
#![deny(unreachable_patterns)]

#[macro_use]
mod error;
mod ast;
mod builtins;
mod compiler;
mod expr;
mod lexer;
mod parser;
mod syntax;
mod tag;
mod token;
mod value;
pub mod vm;

#[cfg(feature = "repl")]
pub mod repl;

pub use {
    ast::AST,
    builtins::builtins,
    compiler::{compile, Code},
    error::{print_error, Error},
    expr::Expr,
    lexer::scan,
    parser::parse,
    syntax::Syntax,
    tag::Tag,
    token::Token,
    value::{Builtin, Value},
    vm::{Scope, VM},
};

pub type Result<T> = std::result::Result<T, Error>;
