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

impl Token {
    pub fn new(kind: TokenKind, pos: usize, len: usize) -> Token {
        Token { kind, pos, len }
    }
}

impl TokenStream {
    pub fn from(source: String, tokens: Vec<Token>) -> TokenStream {
        TokenStream { source, tokens }
    }

    pub fn into_iter(&self) -> TokenIter {
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
