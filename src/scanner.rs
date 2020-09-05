use {
    crate::{token, Result, Syntax, Token, TokenStream},
    std::{iter::Peekable, str::Chars},
};

struct Scanner<'s> {
    tokens: Vec<Token>,         // list we're building
    source: &'s str,            // template source code
    chars: Peekable<Chars<'s>>, // iterator
    pos: usize,                 // current position in `source`
    started: bool,              // scan has begun
    indents: Vec<usize>,        // current depth
    in_tag: usize,              // whether we're inside a <tag> or not
}

/// Scans source code and produces a `TokenStream`.
pub fn scan<S: AsRef<str>>(source: S) -> Result<TokenStream> {
    let source = source.as_ref();
    let mut scanner = Scanner::from(source);
    scanner.scan()?;
    let tokens = scanner.tokens;
    Ok(TokenStream::from(source.to_string(), tokens))
}

impl<'s> Scanner<'s> {
    /// Create a scanner from source code.
    fn from(source: &'s str) -> Scanner<'s> {
        Scanner {
            source,
            tokens: vec![],
            chars: source.chars().peekable(),
            pos: 0,
            in_tag: 0,
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
    fn prev_is(&self, kind: Syntax) -> bool {
        if self.tokens.is_empty() {
            return false;
        }
        self.tokens
            .get(self.tokens.len() - 1)
            .filter(|t| t.kind == kind)
            .is_some()
    }

    /// Advance position in `source` and return next `char`.
    fn next(&mut self) -> Option<char> {
        if self.started {
            self.pos += 1;
        } else {
            self.started = true;
        }
        self.chars.next()
    }

    /// Add single Syntax to tokens list.
    fn append(&mut self, kind: Syntax) -> Result<()> {
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

    /// Turn `source` into vector of `Token`, or error.
    fn scan(&mut self) -> Result<()> {
        while let Some(c) = self.next() {
            let start = self.pos;
            let kind = match c {
                '\n' => self.scan_newline()?,
                ')' | '[' | ']' | '{' | '}' => Syntax::Bracket(c),
                ';' | '#' | '.' | '@' | ':' | '=' | '/' => Syntax::Special(c),
                '"' | '\'' | '`' => self.scan_string(c)?,
                '-' => {
                    if self.peek().filter(|c| c.is_numeric()).is_some() {
                        self.scan_number()?
                    } else {
                        self.scan_word()?
                    }
                }
                '<' => {
                    self.in_tag += 1;
                    Syntax::Bracket('<')
                }
                '>' => {
                    if self.in_tag == 0 {
                        return scan_error!(self.pos, 1, "Got > with no tags open.");
                    }
                    self.in_tag -= 1;
                    Syntax::Bracket('>')
                }
                '(' => {
                    if self.prev_is(Syntax::Special('=')) {
                        let mut open = 0;
                        while let Some(&c) = self.peek() {
                            if c == ')' && open == 0 {
                                self.next();
                                break;
                            } else if c == '(' {
                                open += 1;
                            } else if c == ')' {
                                open -= 1;
                            }
                            self.next();
                        }
                        Syntax::JS
                    } else {
                        Syntax::Bracket('(')
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
            if kind == Syntax::None {
                continue;
            }

            self.tokens
                .push(Token::new(kind, start, self.pos - start + 1));
        }

        // Add final semicolon before EOF, if not present.
        if !self.prev_is(Syntax::Special(';')) && !self.prev_is(Syntax::Dedent) {
            self.append(Syntax::Special(';'))?;
        }

        // Close open indents
        while !self.indents.is_empty() {
            self.indents.pop();
            self.append(Syntax::Dedent)?;
        }

        Ok(())
    }

    /// Parse until we find a non-number.
    fn scan_number(&mut self) -> Result<Syntax> {
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

        Ok(Syntax::Number)
    }

    /// Scan until closing delimiter.
    /// Call when the first char of the string will be `next()`.
    fn scan_string(&mut self, delimiter: char) -> Result<Syntax> {
        let start = self.pos;
        let mut prev = '0'; // TODO: actual escape code parsing
        while let Some(c) = self.next() {
            if c == delimiter && prev != '\\' {
                return Ok(Syntax::String);
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
    fn scan_word(&mut self) -> Result<Syntax> {
        self.eat(|c| !token::RESERVED.contains(&c));
        Ok(Syntax::Word)
    }

    /// Figure out indents and dedents.
    fn scan_newline(&mut self) -> Result<Syntax> {
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
            self.tokens.push(Token::new(Syntax::Indent, start, 1));
            self.indents.push(indent);
            return Ok(Syntax::None);
        }

        // lesser indent than current depth: Dedent
        if indent < last {
            self.append(Syntax::Special(';'))?;
            while self.indents.len() > 0 {
                if indent < self.indents[self.indents.len() - 1] {
                    self.indents.pop();
                    self.append(Syntax::Dedent)?;
                } else {
                    break;
                }
            }
            return Ok(Syntax::None);
        }

        // current depth == current indent
        Ok(Syntax::Special(';'))
    }
}
