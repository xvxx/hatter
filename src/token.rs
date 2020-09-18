use crate::{Error, Result, Syntax};

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

    /// Will it produce a `literal()`?
    pub fn is_literal(&self) -> bool {
        use Syntax::*;
        // We want this to fail if new variants are added.
        match self.kind {
            None | Number | String(..) | Word | JS | Op | Colon | Comma | LParen | RParen
            | LCurly | RCurly | LStaple | RStaple | LessThan | GreaterThan => true,
            Indent | Dedent | Semi => false,
        }
    }

    /// Get the literal value in source code, if it has one.
    pub fn literal(&self) -> &str {
        if self.is_literal() {
            self.lit
        } else {
            ""
        }
    }

    /// Create a string of the literal value.
    pub fn to_string(&self) -> String {
        self.literal().to_string()
    }

    /// Convert into native number or error. No weak typing.
    pub fn to_usize(&self) -> Result<usize> {
        self.literal()
            .parse::<usize>()
            .map_err(|e| Error::new(e.to_string(), self.pos, 1))
    }

    /// Convert into native number or error. No weak typing.
    pub fn to_f64(&self) -> Result<f64> {
        self.literal()
            .parse::<f64>()
            .map_err(|e| Error::new(e.to_string(), self.pos, 1))
    }
}
