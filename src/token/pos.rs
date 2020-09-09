use {
    crate::{Error, Result, Syntax, Token},
    std::{fmt, ops},
};

pub struct TokenPos<'s> {
    source: &'s str,
    tok: &'s Token,
}

impl<'s> ops::Deref for TokenPos<'s> {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        self.tok
    }
}

impl<'s> fmt::Debug for TokenPos<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenPos")
            .field("pos", &self.tok.pos)
            .field("len", &self.tok.len)
            .field("lit", &self.literal())
            .finish()
    }
}

impl<'s> TokenPos<'s> {
    pub fn new(tok: &'s Token, source: &'s str) -> TokenPos<'s> {
        TokenPos { source, tok }
    }

    pub fn literal(&self) -> &str {
        match self.kind {
            Syntax::Indent => "INDENT",
            Syntax::Dedent => "DEDENT",
            Syntax::Special(';') => ";",
            _ => {
                if self.source.len() <= self.pos {
                    ""
                } else {
                    &self.source[self.tok.range()]
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        self.literal().to_string()
    }

    pub fn to_f64(&self) -> Result<f64> {
        self.literal().parse::<f64>().map_err(|err| Error {
            details: format!("{}", err),
            pos: self.pos,
            len: self.len,
        })
    }
}

impl<'s> From<TokenPos<'s>> for String {
    fn from(tok: TokenPos<'s>) -> Self {
        tok.to_string()
    }
}
