//! Hatter is a small, whitespace sensitive templating language with
//! HTML support built right in. Its HTML features and syntax are a
//! cheap knock off of Imba, except Hatter produces raw, static HTML -
//! no JavaScript in sight.

//! Hatter can be used to generate static web sites or to render
//! server side content in a good ol' fashioned web application -
//! either with Vial or your Rust web framework of choice.

//! If you're feeling adventerous, or mad as a hatter, you can use
//! the standalone binary to turn templates into HTML files or
//! include the zero-dependency Rust library in your (web/cli/?)
//! application.

#![deny(unused_must_use)]
#![deny(unreachable_patterns)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::new_without_default)]
#![allow(clippy::inherent_to_string)]

#[macro_use]
mod macros;
#[macro_use]
mod error;
mod compile;
mod eval;
mod interner;
mod omap;
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
    error::{line_and_col, print_error, Error, ErrorKind},
    eval::{
        args::Args,
        builtin,
        env::{eval, render, Env, Jump, Scope},
        value::{self, Fn, Native, Object, Special, Value},
    },
    interner::Interner,
    omap::OMap,
    symbol::Symbol,
    template::Template,
};

/// Alias for a generic Result type.
pub type Result<T> = std::result::Result<T, Error>;
