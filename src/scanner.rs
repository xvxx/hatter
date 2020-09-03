use {
    crate::{token, Result, Token, TokenKind, TokenStream},
    std::{iter::Peekable, str::Chars},
};

struct Scanner<'s> {
    tokens: Vec<Token>,
    source: &'s str,
    chars: Peekable<Chars<'s>>,
    pos: usize,
    started: bool,
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
    /// Create a scanner from source code.
    fn from(source: &'s str) -> Scanner<'s> {
        Scanner {
            source,
            tokens: vec![],
            chars: source.chars().peekable(),
            pos: 0,
            started: false,
        }
    }

    /// Peek at next `char` without iterating.
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Advance position in `src` and return next `char`.
    fn next(&mut self) -> Option<char> {
        if self.started {
            self.pos += 1;
        } else {
            self.started = true;
        }
        self.chars.next()
    }

    /// Consume and discard input while check(peek()) is true.
    /// Return value indicates whether anything was consumed.
    fn eat(&mut self, check: fn(char) -> bool) -> bool {
        let mut eaten = false;
        while self.peek().filter(|&&c| check(c)).is_some() {
            eaten = true;
            self.next();
        }
        eaten
    }

    /// Turn `src` into vector of `Token`, or error.
    fn scan(&mut self) -> Result<()> {
        while let Some(c) = self.next() {
            let start = self.pos;
            let kind = match c {
                '<' | '>' | '(' | ')' | '[' | ']' | '{' | '}' => TokenKind::Bracket(c),
                '#' | '.' | '@' | ':' | '=' => TokenKind::Special(c),
                '"' | '\'' | '`' => self.scan_string(c)?,
                '-' => {
                    if self.peek().filter(|c| c.is_numeric()).is_some() {
                        self.scan_number()?
                    } else {
                        self.scan_word()?
                    }
                }
                _ if c.is_numeric() => self.scan_number()?,
                _ if c.is_whitespace() => {
                    self.eat(|c| c.is_whitespace());
                    continue;
                }
                _ => self.scan_word()?,
            };

            self.tokens.push(Token::new(kind, start, self.pos - start));
        }

        Ok(())
    }

    /// Parse until we find a non-number.
    fn scan_number(&mut self) -> Result<TokenKind> {
        let mut saw_dot = false;

        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '_' {
                self.next();
            } else if !saw_dot && c == '.' {
                saw_dot = true;
                self.next();
            } else {
                break;
            }
        }

        Ok(TokenKind::Number)
    }

    /// Scan until closing delimiter.
    /// Call when the first char of the string will be `next()`.
    fn scan_string(&mut self, delimiter: char) -> Result<TokenKind> {
        let start = self.pos;
        let mut prev = '0'; // TODO: actual escape code parsing
        while let Some(c) = self.next() {
            if c == delimiter && prev != '\\' {
                return Ok(TokenKind::String);
            }
            prev = c;
        }

        scan_error!(
            start,
            1,
            "Unclosed string. Expected closing quote: {}  ",
            delimiter
        )
    }

    /// Parse until we encounter a `token::RESERVED` char.
    fn scan_word(&mut self) -> Result<TokenKind> {
        let start = self.pos;
        self.eat(|c| !token::RESERVED.contains(&c));
        Ok(match &self.source[start..=self.pos] {
            _ => TokenKind::Word,
        })
    }
}
