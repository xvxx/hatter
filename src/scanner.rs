use crate::{Result, Token, TokenStream};

struct Scanner<'s> {
    tokens: Vec<Token>,
    src: &'s str,
}

/// Scans source code and produces a `TokenStream`.
pub fn scan<S: AsRef<str>>(src: S) -> Result<TokenStream> {
    let src = src.as_ref();
    let mut scanner = Scanner::from(src);
    scanner.scan()?;
    let tokens = scanner.tokens;
    Ok(TokenStream::from(src.to_string(), tokens))
}

impl<'s> Scanner<'s> {
    fn from(src: &'s str) -> Scanner<'s> {
        Scanner {
            src,
            tokens: vec![],
        }
    }

    fn scan(&mut self) -> Result<()> {
        Ok(())
    }
}
