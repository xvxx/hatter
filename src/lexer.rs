use {
    crate::{token, Result, Syntax, Token},
    std::{fmt, iter::Peekable, str::CharIndices},
};

struct Lexer<'s> {
    tokens: Vec<Token<'s>>,           // list we're building
    source: &'s str,                  // template source code
    pos: usize,                       // current position in `source`
    indents: Vec<usize>,              // current depth
    chars: Peekable<CharIndices<'s>>, // iterator
    cur: char,                        // current character
    in_tag: bool,                     // parsing <tag>?
    in_container: usize,              // parsing inside [] {} <> ?
    style: Style,                     // tabs or spaces?
}

#[derive(Debug, PartialEq)]
enum Style {
    None,
    Tabs,
    Spaces,
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

/// Scans source code and produces a list of `Token`s.
pub fn scan<'s>(source: &'s str) -> Result<Vec<Token<'s>>> {
    let mut lexer = Lexer::from(source);
    lexer.scan()?;
    Ok(lexer.tokens)
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
            style: Style::None,
            in_tag: false,
            in_container: 0,
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
        self.tokens.push(Token::new(kind, self.pos, 1, ""));
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
                ';' | ',' | '#' | '.' | '@' | '/' => Syntax::Special(c),
                '"' | '\'' | '`' => self.scan_string(c)?,
                ':' => {
                    if self.peek_is('=') {
                        self.next();
                        self.scan_word()?
                    } else {
                        Syntax::Special(c)
                    }
                }
                '=' => {
                    if self.in_tag {
                        Syntax::Special(c)
                    } else if self.peek_is('=') {
                        self.next();
                        self.scan_word()?
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
                    if self.peek_is('!') {
                        while let Some(&c) = self.peek() {
                            self.next();
                            if c == '>' {
                                break;
                            }
                        }
                        Syntax::String
                    } else if self.peek_is('=') || self.peek_is(c) {
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
                '[' | '{' => {
                    self.in_container += 1;
                    Syntax::Bracket(c)
                }
                ']' | '}' => {
                    self.in_container -= 1;
                    Syntax::Bracket(c)
                }
                ')' => Syntax::Bracket(c),
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

            let len = self.pos - start + self.cur.len_utf8();
            self.tokens.push(Token::new(
                kind,
                start,
                len,
                &self.source[start..start + len],
            ));
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
        let mut start = self.pos + 1;
        let mut prev = '0'; // TODO: actual escape code parsing
        let mut triple = false;

        // """ or ''' or ```
        if self.peek_is(delimiter) {
            self.next();
            if self.peek_is(delimiter) {
                self.next();
                triple = true;
                start += 2;
            }
        }

        while let Some(c) = self.next() {
            if c == delimiter && prev != '\\' {
                if !triple {
                    let len = self.pos - start - 1;
                    self.tokens.push(Token::new(
                        Syntax::String,
                        start,
                        len,
                        &self.source[start..=start + len],
                    ));
                    return Ok(Syntax::None);
                } else if self.peek_is(delimiter) {
                    self.next();
                    if self.peek_is(delimiter) {
                        self.next();
                        let len = self.pos - start - 3;
                        self.tokens.push(Token::new(
                            Syntax::String,
                            start,
                            len,
                            &self.source[start..=start + len],
                        ));
                        return Ok(Syntax::None);
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
        if self.in_tag || self.in_container > 0 {
            return Ok(Syntax::Special(';'));
        }

        let start = self.pos;
        let mut indent = 0;
        loop {
            // ignore stacked newlines
            self.eat(|c| c == '\n');

            // tabs vs spaces error checking
            let style = if self.peek_is(' ') {
                Style::Spaces
            } else if self.peek_is('\t') {
                Style::Tabs
            } else {
                Style::None
            };

            if self.style == Style::None {
                self.style = style;
            } else if style != Style::None && self.style != style {
                return scan_error!(
                    start + 1,
                    self.pos - start + 1,
                    "Can't mix tabs and spaces. Expected {:?}, found {:?}",
                    self.style,
                    style
                );
            }

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
            self.tokens.push(Token::new(Syntax::Indent, start, 1, ""));
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
