use std::{fmt, ops};

pub const RESERVED: [char; 17] = [
    '\n', '\t', '\r', ' ', '#', '.', '@', ':', '=', '<', '>', '(', ')', '[', ']', '{', '}',
];

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub len: usize,
    pub kind: Syntax,
}

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    source: String,
    pos: usize,
}

pub struct TokenPos<'s> {
    source: &'s str,
    tok: &'s Token,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Syntax {
    None,
    Indent,
    Dedent,
    Bracket(char), // < > ( ) [ ] { }
    Special(char), // ; # . @ : = /
    Bool,
    Number,
    String,
    Word,
    JS, // (JavaScript)
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
}

impl<'s> From<TokenPos<'s>> for String {
    fn from(tok: TokenPos<'s>) -> Self {
        tok.to_string()
    }
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
                start..start + self.len - 1
            }
            _ => self.pos..self.pos + self.len,
        }
    }
}

impl TokenStream {
    pub fn from(source: String, tokens: Vec<Token>) -> TokenStream {
        TokenStream {
            source,
            tokens,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<TokenPos> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(TokenPos {
                tok: self.tokens.get(self.pos)?,
                source: &self.source,
            })
        }
    }

    pub fn peek2(&self) -> Option<TokenPos> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(TokenPos {
                tok: self.tokens.get(self.pos + 1)?,
                source: &self.source,
            })
        }
    }

    pub fn next(&mut self) -> Option<TokenPos> {
        if self.tokens.is_empty() {
            None
        } else {
            let pos = self.pos;
            self.pos += 1;
            Some(TokenPos {
                tok: self.tokens.get(pos)?,
                source: &self.source,
            })
        }
    }
}
