//! The lexer turns a string of source code into Tokens.

use {
    crate::{Result, Syntax, SyntaxTrait, Token},
    std::{iter::Peekable, mem, str::CharIndices},
};

struct Lexer<'s> {
    tokens: Vec<Token<'s>>,           // what we're building
    source: &'s str,                  // template source code
    pos: usize,                       // current position in `source`
    indents: Vec<usize>,              // current depth
    chars: Peekable<CharIndices<'s>>, // iterator
    cur: char,                        // current character
    mode: Mode,                       // lexing mode
    modes: Vec<Mode>,                 // stack of recent modes
    style: Style,                     // tabs or spaces?
}

#[derive(Debug, PartialEq)]
enum Mode {
    None,      // Regular
    Container, // [List] or {Map} or (Sub-Expr)
    Tag,       // <Tag>
}

#[derive(Debug, PartialEq)]
enum Style {
    None,
    Tabs,
    Spaces,
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
            mode: Mode::None,
            modes: vec![],
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

    /// Did we just see =?
    fn prev_is_equal_sign(&self) -> bool {
        if self.tokens.is_empty() {
            return false;
        }
        self.tokens
            .get(self.tokens.len() - 1)
            .filter(|t| t.literal() == "=")
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

    /// Change the lexing mode.
    fn set_mode(&mut self, mode: Mode) {
        self.modes.push(mem::replace(&mut self.mode, mode));
    }

    /// Set lexing mode to most recent mode, i.e. pop the mode stack.
    fn pop_mode(&mut self) {
        if !self.modes.is_empty() {
            self.mode = self.modes.remove(self.modes.len() - 1);
        }
    }

    /// Are we lexing a tag?
    fn in_tag(&self) -> bool {
        matches!(self.mode, Mode::Tag)
    }

    /// Are we lexing a container?
    fn in_container(&self) -> bool {
        matches!(self.mode, Mode::Container)
    }

    /// Turn `source` into vector of `Token`, or error.
    fn scan(&mut self) -> Result<()> {
        while let Some(c) = self.next() {
            let start = self.pos;
            let kind = match c {
                '\n' => self.scan_newline()?,
                ';' => Syntax::Semi,
                ',' => Syntax::Comma,
                '"' | '\'' | '`' => self.scan_string(c)?,
                '!' => {
                    if self.peek_is('!') {
                        Syntax::Op
                    } else {
                        self.scan_op()?
                    }
                }

                ':' => {
                    if self.peek_is('=') || self.in_tag() {
                        self.scan_op()?
                    } else {
                        Syntax::Colon
                    }
                }

                '-' => {
                    if self.peek().filter(|c| c.is_numeric()).is_some() {
                        self.scan_number()?
                    } else {
                        self.scan_op()?
                    }
                }

                '#' => {
                    if self
                        .peek()
                        .filter(|c| **c == '{' || c.is_alphabetic())
                        .is_some()
                    {
                        self.scan_op()?
                    } else {
                        self.scan_comment()?
                    }
                }

                '[' => {
                    self.set_mode(Mode::Container);
                    Syntax::LStaple
                }
                ']' => {
                    self.pop_mode();
                    Syntax::RStaple
                }

                '(' => {
                    if self.in_tag() {
                        self.scan_open_paren_in_tag()?
                    } else {
                        self.set_mode(Mode::Container);
                        Syntax::LParen
                    }
                }
                ')' => {
                    if !self.in_tag() {
                        self.pop_mode();
                    }
                    Syntax::RParen
                }

                '{' => {
                    if self.in_tag() {
                        self.scan_word(true)?
                    } else {
                        self.set_mode(Mode::Container);
                        Syntax::LCurly
                    }
                }
                '}' => {
                    self.pop_mode();
                    Syntax::RCurly
                }

                '<' => self.scan_left_arrow()?,
                '>' => {
                    if self.in_tag() {
                        self.mode = Mode::None;
                        Syntax::RCaret
                    } else {
                        self.scan_op()?
                    }
                }
                '=' => {
                    if self.in_tag() {
                        Syntax::Equal
                    } else {
                        self.scan_op()?
                    }
                }
                '/' => {
                    if self.in_tag() {
                        Syntax::Slash
                    } else {
                        self.scan_op()?
                    }
                }

                _ if c.is_whitespace() => {
                    self.eat(|c| c.is_whitespace());
                    continue;
                }

                _ if c.is_numeric() => self.scan_number()?,
                _ if c.is_alphabetic() || c == '_' => self.scan_word(false)?,
                _ => self.scan_op()?,
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
        if !self.prev_is(Syntax::Semi) && !self.prev_is(Syntax::Dedent) {
            self.append(Syntax::Semi)?;
        }

        // Close open indents
        while !self.indents.is_empty() {
            self.indents.pop();
            self.append(Syntax::Dedent)?;
        }

        // Trim leading ;
        while !self.tokens.is_empty() && self.tokens[0].kind == Syntax::Semi {
            self.tokens.remove(0);
        }

        Ok(())
    }

    /// Scan a single line comment.
    fn scan_comment(&mut self) -> Result<Syntax> {
        self.eat(|c| c != '\n');
        Ok(Syntax::None)
    }

    /// Scan until we find a non-number.
    fn scan_number(&mut self) -> Result<Syntax> {
        let mut saw_dot = false;

        macro_rules! match_set {
            ($( $pattern:pat )|+) => {{
                self.next();
                while let Some(&c) = self.peek() {
                    if matches!(c, $( $pattern )|+) {
                        self.next();
                    } else {
                        break;
                    }
                }
                return Ok(Syntax::Number);
            }};
        }

        // check for 0xdead and stuff
        if self.cur == '0' {
            if let Some(&c) = self.peek() {
                match c {
                    'b' => match_set!('0' | '1' | '_'),
                    'o' => match_set!('0'..='7' | '_'),
                    'x' => match_set!('a'..='f' | '0'..='9'),
                    _ => {}
                }
            }
        }

        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '_' {
                self.next();
            } else if !saw_dot && c == '.' {
                let mut iter = self.chars.clone();
                iter.next();
                if let Some((_, c)) = iter.next() {
                    if c.is_numeric() {
                        saw_dot = true;
                        self.next();
                        continue;
                    }
                }
                break;
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
            } else {
                // empty string
                self.tokens
                    .push(Token::new(Syntax::String(false), start - 1, 2, ""));
                return Ok(Syntax::None);
            }
        }

        while let Some(c) = self.next() {
            if c == delimiter && prev != '\\' {
                if !triple {
                    let len = self.pos - start - 1;
                    let lit = &self.source[start..=start + len];
                    self.tokens.push(Token::new(
                        Syntax::String(delimiter == '"' && lit.contains('{')),
                        start,
                        len,
                        lit,
                    ));
                    return Ok(Syntax::None);
                } else if self.peek_is(delimiter) {
                    self.next();
                    if self.peek_is(delimiter) {
                        self.next();
                        // empty string
                        if self.pos == start + 2 {
                            self.tokens
                                .push(Token::new(Syntax::String(false), start - 3, 6, ""));
                            return Ok(Syntax::None);
                        }
                        let len = self.pos - start - 3;
                        self.tokens.push(Token::new(
                            Syntax::String(false),
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

    /// Scan a word, which may have {interpolation.with(some, whitespace)}.
    fn scan_op(&mut self) -> Result<Syntax> {
        self.eat(|c| c.is_op());
        Ok(Syntax::Op)
    }

    /// Scan a word, which may have {interpolation.with(some, whitespace)}.
    /// Set `in_code` to `true` if the first char of the word is `{`
    fn scan_word(&mut self, mut in_code: bool) -> Result<Syntax> {
        let start = self.pos;
        let mut curlies = 0;

        while let Some(&c) = self.peek() {
            if in_code {
                if c == '}' {
                    if curlies == 0 {
                        in_code = false;
                    } else {
                        curlies -= 0;
                    }
                } else if c == '{' {
                    curlies += 1;
                }
            } else {
                if c == '{' {
                    in_code = true;
                } else if !c.is_word_char() {
                    break;
                }
            }
            self.next();
        }

        if self.peek_is('?') {
            self.next();
        }

        Ok(match &self.source[start..=self.pos] {
            "def" => Syntax::Def,
            "do" => Syntax::Do,
            "return" => Syntax::Return,
            "if" => Syntax::If,
            "then" => Syntax::Then,
            "else" => Syntax::Else,
            "for" => Syntax::For,
            "while" => Syntax::While,
            "in" => Syntax::In,
            "fn" => Syntax::Fn,
            "true" => Syntax::Bool(true),
            "false" => Syntax::Bool(false),
            _ => Syntax::Word,
        })
    }

    /// Determines if < is opening a tag or just a regular `<` sign
    fn scan_left_arrow(&mut self) -> Result<Syntax> {
        let p = *self.peek().unwrap_or(&'0');
        if !self.in_tag() && p.is_tag_opener() {
            // <tag>
            self.mode = Mode::Tag;
            Ok(Syntax::LCaret)
        } else if !self.in_tag() && p == '!' {
            self.next(); // skip !

            let mut comment = false;
            if self.peek_is('-') {
                self.next();
                if self.peek_is('-') {
                    self.next();
                    comment = true
                }
            }

            while let Some(c) = self.next() {
                if comment && c == '-' {
                    if let Some('-') = self.next() {
                        if let Some('>') = self.next() {
                            break;
                        }
                    }
                } else if c == '>' {
                    self.next();
                    break;
                }
            }

            Ok(Syntax::String(true))
        } else {
            // <= << <
            self.scan_op()
        }
    }

    /// Scan an open paren `(` seen in a tag declaration.
    fn scan_open_paren_in_tag(&mut self) -> Result<Syntax> {
        if self.prev_is_equal_sign() {
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
            Ok(Syntax::JS)
        } else {
            self.set_mode(Mode::Container);
            Ok(Syntax::LParen)
        }
    }

    /// Figure out indents and dedents.
    fn scan_newline(&mut self) -> Result<Syntax> {
        if self.in_tag() || self.in_container() {
            return Ok(Syntax::Semi);
        }

        let start = self.pos;
        let mut indent = 0;
        loop {
            // ignore stacked newlines
            self.eat(|c| c == '\n');

            // tabs vs spaces error checking
            self.check_indent_style()?;

            // count indent
            while self.peek_is(' ') || self.peek_is('\t') {
                indent += 1;
                self.next();
            }

            // start over if we hit another newline or a comment
            match self.peek() {
                Some('\n') => {}
                Some('#') => {
                    self.scan_comment()?;
                }
                _ => break,
            }
            indent = 0;
        }

        // EOF
        if self.peek().is_none() {
            return Ok(Syntax::None);
        }

        // if the next token is an operator, continue the last line
        if self.peek().filter(|c| c.is_op() && c != &&'<').is_some() {
            return Ok(Syntax::None);
        }

        // what indent level are we at?
        let last = if !self.indents.is_empty() {
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
            self.append(Syntax::Semi)?;
            while !self.indents.is_empty() {
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
        Ok(Syntax::Semi)
    }

    /// Make sure tabs vs spaces use is consistent.
    fn check_indent_style(&mut self) -> Result<()> {
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
                self.pos,
                1,
                "Can't mix tabs and spaces. Expected {:?}, found {:?}",
                self.style,
                style
            );
        }

        Ok(())
    }
}
