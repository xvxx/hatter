use {
    crate::{Error, Result, Syntax},
    std::{fmt, ops},
};

#[derive(Debug, Clone, Copy)]
pub struct Token<'s> {
    pub pos: usize,
    pub len: usize,
    pub kind: Syntax,
    lit: &'s str,
}

impl<'s> Token<'s> {
    /// Create a Token.
    pub fn new(kind: Syntax, pos: usize, len: usize, lit: &'s str) -> Token<'s> {
        Token {
            kind,
            pos,
            len,
            lit,
        }
    }

    /// Get the literal value in source code, if it has one.
    pub fn literal(&self) -> &str {
        match self.kind {
            Syntax::Dedent | Syntax::Indent => "",
            Syntax::Special(..) | Syntax::Bracket(..) => "",
            _ => self.lit,
        }
    }

    /// Create a string of the literal value.
    pub fn to_string(&self) -> String {
        self.literal().to_string()
    }

    /// Convert into native number or error. No weak typing.
    pub fn to_f64(&self) -> Result<f64> {
        self.literal()
            .parse::<f64>()
            .map_err(|e| Error::new(e.to_string(), self.pos, 1))
    }
}
