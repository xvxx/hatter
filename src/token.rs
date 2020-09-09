use {
    crate::{Error, Result, Syntax},
    std::{fmt, ops},
};

mod pos;
mod stream;

pub use pos::TokenPos;
pub use stream::TokenStream;

pub const RESERVED: [char; 19] = [
    '\n', '\t', '\r', ' ', ';', ',', '#', '.', '@', ':', '/', '<', '>', '(', ')', '[', ']', '{',
    '}',
];

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub len: usize,
    pub kind: Syntax,
}

impl Token {
    /// Create a Token.
    pub fn new(kind: Syntax, pos: usize, len: usize) -> Token {
        Token { kind, pos, len }
    }

    /// Location in source code.
    pub fn range(&self) -> std::ops::Range<usize> {
        match self.kind {
            Syntax::JS | Syntax::String => {
                let start = self.pos + 1;
                start..start + self.len - 2
            }
            _ => self.pos..self.pos + self.len,
        }
    }
}
