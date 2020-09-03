use std::fmt;

pub const RESERVED: [char; 17] = [
    '\n', '\t', '\r', ' ', '#', '.', '@', ':', '=', '<', '>', '(', ')', '[', ']', '{', '}',
];

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    source: String,
}

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub len: usize,
    pub kind: TokenKind,
}

#[derive(Debug)]
pub enum TokenKind {
    None,
    Bracket(char), // < > ( ) [ ] { }
    Special(char), // # . @ : =
    Bool,
    Number,
    String,
    Word,
    // keywords
    If,
    For,
    Do,
    End,
}

pub struct TokenIter<'s> {
    src: &'s str,
    toks: &'s [Token],
    pos: usize,
}

pub struct TokenPos<'s> {
    src: &'s str,
    tok: &'s Token,
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
    fn literal(&self) -> &str {
        let pos = self.tok.pos;
        if self.src.len() > pos {
            &self.src[self.tok.range()]
        } else {
            ""
        }
    }
}

impl Token {
    /// Create a Token.
    pub fn new(kind: TokenKind, pos: usize, len: usize) -> Token {
        Token { kind, pos, len }
    }

    /// Location in source code.
    pub fn range(&self) -> std::ops::Range<usize> {
        self.pos..self.pos + self.len
    }
}

impl TokenStream {
    pub fn from(source: String, tokens: Vec<Token>) -> TokenStream {
        TokenStream { source, tokens }
    }
}

impl<'s> IntoIterator for &'s TokenStream {
    type Item = TokenPos<'s>;
    type IntoIter = TokenIter<'s>;

    fn into_iter(self) -> TokenIter<'s> {
        TokenIter {
            src: &self.source,
            toks: &self.tokens,
            pos: 0,
        }
    }
}

impl<'s> Iterator for TokenIter<'s> {
    type Item = TokenPos<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.toks.len() {
            None
        } else {
            let pos = self.pos;
            self.pos += 1;
            Some(TokenPos {
                src: self.src,
                tok: &self.toks[pos],
            })
        }
    }
}
