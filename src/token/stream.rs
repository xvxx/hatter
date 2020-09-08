use crate::{Token, TokenPos};

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    source: String,
    pos: usize,
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
            Some(TokenPos::new(self.tokens.get(self.pos)?, &self.source))
        }
    }

    pub fn peek2(&self) -> Option<TokenPos> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(TokenPos::new(self.tokens.get(self.pos + 1)?, &self.source))
        }
    }

    pub fn current(&self) -> Option<TokenPos> {
        if self.tokens.is_empty() || self.pos == 0 {
            None
        } else {
            Some(TokenPos::new(self.tokens.get(self.pos - 1)?, &self.source))
        }
    }

    pub fn next(&mut self) -> Option<TokenPos> {
        if self.tokens.is_empty() {
            None
        } else {
            let pos = self.pos;
            self.pos += 1;
            Some(TokenPos::new(self.tokens.get(pos)?, &self.source))
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
