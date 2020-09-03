use {
    crate::{token, Result, Token, TokenKind, TokenStream},
    std::{iter::Peekable, str::Chars},
};

struct Scanner<'s> {
    tokens: Vec<Token>,         // list we're building
    source: &'s str,            // template source code
    chars: Peekable<Chars<'s>>, // iterator
    pos: usize,                 // current position in `source`
    started: bool,              // scan has begun
    indents: Vec<usize>,        // current depth
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
            indents: vec![],
        }
    }

    /// Peek at next `char` without iterating.
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Check the next char.
    fn peek_is(&mut self, c: char) -> bool {
        self.peek().filter(|&&p| p == c).is_some()
    }

    /// Check the type of the just-created token.
    fn prev_is(&self, kind: TokenKind) -> bool {
        if self.tokens.is_empty() {
            return false;
        }
        self.tokens
            .get(self.tokens.len() - 1)
            .filter(|t| t.kind == kind)
            .is_some()
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

    /// Add single TokenKind to tokens list.
    fn append(&mut self, kind: TokenKind) -> Result<()> {
        self.tokens.push(Token::new(kind, self.pos, 1));
        Ok(())
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
                '\n' => self.scan_newline()?,
                '<' | '>' | '(' | ')' | '[' | ']' | '{' | '}' => TokenKind::Bracket(c),
                ';' | '#' | '.' | '@' | ':' | '=' | '/' => TokenKind::Special(c),
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

            // skip empty tokens
            if kind == TokenKind::None {
                continue;
            }

            self.tokens
                .push(Token::new(kind, start, self.pos - start + 1));
        }

        // Add final semicolon before EOF, if not present.
        if !self.prev_is(TokenKind::Special(';')) && !self.prev_is(TokenKind::Dedent) {
            self.append(TokenKind::Special(';'))?;
        }

        // Close open indents
        while !self.indents.is_empty() {
            self.indents.pop();
            self.append(TokenKind::Dedent)?;
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
        self.eat(|c| !token::RESERVED.contains(&c));
        Ok(TokenKind::Word)
    }

    /// Figure out indents and dedents.
    fn scan_newline(&mut self) -> Result<TokenKind> {
        let start = self.pos;
        let mut indent = 0;
        loop {
            // ignore stacked newlines
            self.eat(|c| c == '\n');

            // count indent
            while self.peek_is(' ') || self.peek_is('\t') {
                indent += 1;
                self.next();
            }

            // start over if we hit another newline
            if let Some('\n') = self.peek() {
                indent = 0;
                continue;
            }

            break;
        }

        // what indent level are we at?
        let last = if self.indents.len() > 0 {
            self.indents[self.indents.len() - 1]
        } else {
            0
        };

        // greater indent than current depth: Indent
        if indent > last {
            // set pos to first \n we saw, we may have skipped some
            self.tokens.push(Token::new(TokenKind::Indent, start, 1));
            self.indents.push(indent);
            return Ok(TokenKind::None);
        }

        // lesser indent than current depth: Dedent
        if indent < last {
            self.append(TokenKind::Special(';'))?;
            while self.indents.len() > 0 {
                if indent < self.indents[self.indents.len() - 1] {
                    self.indents.pop();
                    self.append(TokenKind::Dedent)?;
                } else {
                    break;
                }
            }
            return Ok(TokenKind::None);
        }

        // current depth == current indent
        Ok(TokenKind::Special(';'))
    }
}
