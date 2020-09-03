pub struct Tokens {
    tokens: Vec<Token>,
    source: String,
}

pub struct Token {
    pos: usize,
    len: usize,
    kind: TokenKind,
}

pub enum TokenKind {
    None,
    Bool,
    Number,
    String,
    Word,
}

struct TokenIter<'s> {
    src: &'s str,
    toks: &'s [Token],
    pos: usize,
}

struct TokenPos<'s> {
    src: &'s str,
    tok: &'s Token,
}

impl Tokens {
    pub fn from(source: String, tokens: Vec<Token>) -> Tokens {
        Tokens { source, tokens }
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
