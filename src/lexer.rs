use {
    crate::{token, Result, Syntax, Token, TokenStream},
    std::{iter::Peekable, str::CharIndices},
};

struct Lexer<'s> {
    tokens: Vec<Token>,               // list we're building
    source: &'s str,                  // template source code
    pos: usize,                       // current position in `source`
    indents: Vec<usize>,              // current depth
    chars: Peekable<CharIndices<'s>>, // iterator
    cur: char,                        // current character
    in_tag: bool,                     // parsing <tag>?
}

trait Reserved {
    fn is_reserved(&self) -> bool;
}

impl Reserved for char {
    #[rustfmt::skip]
    fn is_reserved(&self) -> bool {
        matches!(self, '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' |
            ';' | ',' | '.' | '#' | '@' | ':' | '=')
    }
}

/// Scans source code and produces a `TokenStream`.
pub fn scan<S: AsRef<str>>(source: S) -> Result<TokenStream> {
    let source = source.as_ref();
    let mut lexer = Lexer::from(source);
    lexer.scan()?;
    let tokens = lexer.tokens;
    Ok(TokenStream::from(source.to_string(), tokens))
}

impl<'s> Lexer<'s> {
    /// Create a lexer from source code.
    fn from(source: &'s str) -> Lexer<'s> {
        Lexer {
            source,
            tokens: vec![],
            chars: source.char_indices().peekable(),
            pos: 0,
            indents: vec![],
            cur: '0',
            in_tag: false,
        }
    }

    /// Peek at next `char` without iterating.
    fn peek(&mut self) -> Option<&char> {
        if let Some((_, c)) = self.chars.peek() {
            Some(c)
        } else {
            None
        }
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
        if let Some((pos, c)) = self.chars.next() {
            self.pos = pos;
            self.cur = c;
            Some(c)
        } else {
            None
        }
    }

    /// Add single Syntax to tokens list.
    fn append(&mut self, kind: Syntax) -> Result<()> {
        self.tokens.push(Token::new(kind, self.pos, 1));
        Ok(())
    }

    /// Consume and discard input while check(peek()) is true.
    /// Return value indicates whether anything was consumed.
    fn eat(&mut self, check: impl Fn(char) -> bool) -> bool {
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
                ';' | ',' | '#' | '.' | '@' | '/' => Syntax::Special(c),
                '"' | '\'' | '`' => self.scan_string(c)?,
                ':' => {
                    if self.peek_is('=') {
                        self.next();
                        Syntax::Word
                    } else {
                        Syntax::Special(c)
                    }
                }
                '=' => {
                    if self.in_tag {
                        Syntax::Special(c)
                    } else if self.peek_is('=') {
                        self.next();
                        Syntax::Word
                    } else {
                        self.scan_word()?
                    }
                }
                '-' => {
                    if self.peek().filter(|c| c.is_numeric()).is_some() {
                        self.scan_number()?
                    } else {
                        self.scan_word()?
                    }
                }
                '<' => {
                    if self.peek_is('=') || self.peek_is(c) {
                        self.next(); // skip =
                        self.scan_word()?
                    } else if self.in_tag || self.peek_is(' ') {
                        self.scan_word()?
                    } else {
                        self.in_tag = true;
                        Syntax::Bracket('<')
                    }
                }
                '>' => {
                    if self.peek_is('=') || self.peek_is(c) {
                        self.next(); // skip =
                        self.scan_word()?
                    } else if self.in_tag {
                        self.in_tag = false;
                        Syntax::Bracket('>')
                    } else {
                        self.scan_word()?
                    }
                }
                '(' => {
                    if self.in_tag && self.prev_is(Syntax::Special('=')) {
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

            let end = self.pos - start + self.cur.len_utf8();
            self.tokens.push(Token::new(kind, start, end));
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
        let mut triple = false;

        // """ or ''' or ```
        if self.peek_is(delimiter) {
            self.next();
            if self.peek_is(delimiter) {
                self.next();
                triple = true;
            }
        }

        while let Some(c) = self.next() {
            if c == delimiter && prev != '\\' {
                if !triple {
                    return Ok(Syntax::String);
                } else if self.peek_is(delimiter) {
                    self.next();
                    if self.peek_is(delimiter) {
                        self.next();
                        return Ok(Syntax::TripleString);
                    }
                }
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
        self.eat(|c| !c.is_reserved() && !c.is_whitespace());
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
