use crate::{scan, Error, Result, Stmt, Syntax, Tag, Token};

#[cfg(debug_assertions)]
const STACK_SIZE: usize = 1000; // infinite loop protection

#[derive(Debug)]
pub struct Parser<'s, 't> {
    tokens: &'t [Token<'s>], // code
    ast: Vec<Stmt>,          // what we're building
    pos: usize,              // position in tokens vec
    tags: usize,             // open tags

    #[cfg(debug_assertions)]
    peeked: usize, // infinite loop protection hack
}

pub fn parse<'t>(tokens: &'t [Token]) -> Result<Vec<Stmt>> {
    let mut parser = Parser::from(tokens);
    parser.parse()?;
    Ok(parser.ast)
}

impl<'s, 't> Parser<'s, 't> {
    /// Create a `Parser` from a `TokenStream`.
    pub fn from(tokens: &'t [Token<'s>]) -> Parser<'s, 't> {
        Parser {
            tokens,
            ast: vec![],
            tags: 0,
            pos: 0,

            #[cfg(debug_assertions)]
            peeked: 0,
        }
    }

    /// Parse `TokenStream` into `AST`.
    pub fn parse(&mut self) -> Result<()> {
        while !self.peek_eof() {
            let stmt = self.stmt()?;
            self.ast.push(stmt);
            self.eat(Syntax::Semi);
        }
        Ok(())
    }

    /// Peek at next `Token`.
    fn peek(&mut self) -> Option<Token> {
        #[cfg(debug_assertions)]
        {
            self.peeked += 1;
            if self.peeked > STACK_SIZE {
                panic!("infinite loop while peek()ing: {:?}", self.tokens.get(0));
            }
        }
        self.tokens.get(self.pos).map(|t| *t)
    }

    /// Peek two ahead.
    fn peek2(&mut self) -> Option<Token> {
        #[cfg(debug_assertions)]
        {
            self.peeked += 1;
            if self.peeked > STACK_SIZE {
                panic!("infinite loop while peek()ing: {:?}", self.tokens.get(0));
            }
        }
        self.tokens.get(self.pos + 1).map(|t| *t)
    }

    /// Get the next token's kind.
    fn peek_kind(&mut self) -> Syntax {
        self.peek().map(|t| t.kind).unwrap_or(Syntax::None)
    }

    /// Check the next token's kind.
    fn peek_is(&mut self, kind: Syntax) -> bool {
        self.peek_kind() == kind
    }

    /// Check kind of the token after peek().
    fn peek2_is(&mut self, kind: Syntax) -> bool {
        self.peek2().filter(|t| t.kind == kind).is_some()
    }

    /// Will self.next() deliver EOF?
    fn peek_eof(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Advance iterator an return next `Token`.
    fn try_next(&mut self) -> Option<Token> {
        if !self.tokens.is_empty() {
            Some(self.next())
        } else {
            None
        }
    }

    /// Move iterator back.
    fn back(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    /// Advance iterator an return next `Token`.
    fn next(&mut self) -> Token {
        #[cfg(debug_assertions)]
        {
            self.peeked = 0;
        }
        let pos = self.pos;
        self.pos += 1;
        *self.tokens.get(pos).unwrap()
    }

    /// Skip one token.
    fn skip(&mut self) {
        let _ = self.next();
    }

    /// Skip all tokens of `kind`.
    fn eat(&mut self, kind: Syntax) {
        while self.peek_is(kind) {
            self.skip();
        }
    }

    /// Trigger parse error for next() token.
    fn error<T, S: AsRef<str>>(&mut self, msg: S) -> Result<T> {
        use crate::ErrorKind::ParseError;

        Err(if let Some(got) = self.try_next() {
            Error::new(
                ParseError,
                format!("expected {}, got {:?}", msg.as_ref(), got.kind),
                got.pos,
                got.len,
            )
        } else {
            Error::new(
                ParseError,
                format!("expected {}, got EOF", msg.as_ref()),
                0,
                0,
            )
        })
    }

    /// Consumes and returns the next token if it's of `kind`,
    /// otherwise errors.
    fn expect(&mut self, kind: Syntax) -> Result<Token> {
        if self.peek_kind() == kind {
            Ok(self.next())
        } else {
            self.error(format!("{:?}", kind))
        }
    }

    /// Parse a number.
    fn number(&mut self) -> Result<Stmt> {
        Ok(Stmt::Number(self.expect(Syntax::Number)?.to_isize()?))
    }

    /// Parse a string.
    fn string(&mut self) -> Result<Stmt> {
        let tok = self.next();
        let is_interpolated = match tok.kind {
            Syntax::String(is) => is,
            Syntax::Word => true,
            _ => return self.error("String"),
        };

        let lit = tok.to_string();
        if is_interpolated && lit.contains('{') {
            let mut parts = vec![];
            let mut idx = 0;
            while let Some(i) = lit[idx..].find('{') {
                // check for escaped \{}
                if i > 0 && lit[idx..].bytes().nth(i - 1).unwrap_or(b'0') == b'\\' {
                    parts.push(Stmt::String(lit[idx..i + idx - 1].into()));
                    parts.push(Stmt::String(lit[idx + i..i + idx + 1].into()));
                    idx += i + 1;
                    continue;
                }

                {
                    let s = &lit[idx..i + idx];
                    if !s.is_empty() {
                        parts.push(Stmt::String(s.into()));
                    }
                }
                idx += i + 1;
                let mut end = idx;
                for (x, b) in lit[idx..].bytes().enumerate() {
                    if b == b'}' {
                        end = idx + x;
                        break;
                    }
                }
                // What! Rust 'lifetime magic.
                let mut exprs = scan(&lit[idx..end])
                    .and_then(|t| parse(&t))
                    .map_err(|mut e| {
                        e.pos += tok.pos + idx - 1; // probably not right yet...
                        e
                    })?;
                parts.append(&mut exprs);
                idx = end + 1;
            }
            if idx < lit.len() {
                parts.push(Stmt::String(lit[idx..].into()));
            }
            if parts.len() == 1 {
                Ok(parts.remove(0))
            } else {
                Ok(Stmt::Call("concat".into(), parts))
            }
        } else {
            Ok(Stmt::String(lit))
        }
    }

    /// Parse a word.
    fn word(&mut self) -> Result<Stmt> {
        let word = self.expect(Syntax::Word)?;
        Ok(match word.literal() {
            "true" | "false" => Stmt::Bool(word.literal() == "true"),
            _ => Stmt::Word(word.to_string()),
        })
    }

    /// Parse a function literal.
    fn fn_literal(&mut self) -> Result<Stmt> {
        self.expect(Syntax::LParen)?;
        let mut args = vec![];
        while !self.peek_is(Syntax::RParen) {
            args.push(self.expect(Syntax::Word)?.to_string());
            if self.peek_is(Syntax::Comma) {
                self.next();
            } else {
                break;
            }
        }
        self.expect(Syntax::RParen)?;
        if self.peek_is(Syntax::Indent) {
            Ok(Stmt::Fn(args, self.block()?))
        } else {
            Ok(Stmt::Fn(args, vec![self.stmt()?]))
        }
    }

    /// Parse a code expression.
    fn expr(&mut self) -> Result<Stmt> {
        let left = self.atom()?;

        if !self.peek_is(Syntax::Op) {
            return Ok(left);
        }

        let next = self.peek().unwrap();
        let lit = next.literal();
        match lit {
            ":=" | "=" => {
                let reassign = lit == "=";
                self.skip(); // skip op
                if let Stmt::Word(name) = left {
                    Ok(Stmt::Assign(name, bx!(self.expr()?), reassign))
                } else {
                    self.error("Word")
                }
            }
            "." => {
                // convert word to str, ex: map.key => .(map, "key")
                self.skip();
                let right = self.expr()?;
                if let Stmt::Word(word) = right {
                    Ok(Stmt::Call(".".into(), vec![left, Stmt::String(word)]))
                } else {
                    Ok(Stmt::Call(".".into(), vec![left, right]))
                }
            }
            _ => {
                // check for += and friends
                if !matches!(lit, "==" | "!=" | ">=" | "<=")
                    && matches!(lit.bytes().last(), Some(b'='))
                {
                    let op = left.to_string();
                    self.skip();
                    Ok(Stmt::Assign(
                        op.clone(),
                        bx!(Stmt::Call(op, vec![left, self.expr()?])),
                        true, // reassignment
                    ))
                } else {
                    Ok(Stmt::Call(
                        self.next().to_string(),
                        vec![left, self.expr()?],
                    ))
                }
            }
        }
    }

    /// Parse an indivisible unit, as the Ancient Greeks would say.
    fn atom(&mut self) -> Result<Stmt> {
        match self.peek_kind() {
            // Literal
            Syntax::String(..) => Ok(self.string()?),
            Syntax::Number => Ok(self.number()?),
            // Tag
            Syntax::LCaret => self.tag(),
            // Sub-expression
            Syntax::LParen => {
                self.skip();
                let expr = self.expr()?;
                self.expect(Syntax::RParen)?;
                Ok(expr)
            }
            // List
            Syntax::LStaple => {
                self.skip();
                self.eat(Syntax::Semi);
                let mut list = vec![];
                while !self.peek_eof() && !self.peek_is(Syntax::RStaple) {
                    self.eat(Syntax::Semi);
                    list.push(self.expr()?);
                    if self.peek_is(Syntax::RStaple) {
                        break;
                    } else if self.peek_is(Syntax::Semi) {
                        self.eat(Syntax::Semi);
                    } else {
                        self.expect(Syntax::Comma)?;
                    }
                }
                self.eat(Syntax::Semi);
                self.expect(Syntax::RStaple)?;
                Ok(Stmt::List(list))
            }
            // Map
            Syntax::LCurly => {
                self.skip();
                self.eat(Syntax::Semi);
                let mut map = vec![];
                while !self.peek_eof() && !self.peek_is(Syntax::RCurly) {
                    self.eat(Syntax::Semi);
                    let key = match self.peek_kind() {
                        Syntax::Word | Syntax::String(..) | Syntax::Number => {
                            self.next().to_string()
                        }
                        _ => return self.error("String key name"),
                    };
                    self.expect(Syntax::Colon)?;
                    self.eat(Syntax::Semi);
                    let val = self.expr()?;
                    map.push((key, val));
                    if self.peek_is(Syntax::Semi) {
                        self.eat(Syntax::Semi);
                    } else if self.peek_is(Syntax::RCurly) {
                        break;
                    } else {
                        self.expect(Syntax::Comma)?;
                    }
                }
                self.eat(Syntax::Semi);
                self.expect(Syntax::RCurly)?;
                Ok(Stmt::Map(map))
            }
            // Variables and function calls
            Syntax::Word => {
                let word = self.word()?;

                // check for "fn()" literal
                if let Stmt::Word(w) = &word {
                    if w == "fn" {
                        return self.fn_literal();
                    }
                }

                if !self.peek_is(Syntax::LParen) {
                    return Ok(word);
                } else {
                    let name = word.to_string();
                    Ok(Stmt::Call(name, self.args()?))
                }
            }
            _ => self.error("Atom"),
        }
    }

    /// Parse (args) part of a function call.
    fn args(&mut self) -> Result<Vec<Stmt>> {
        self.expect(Syntax::LParen)?;
        if self.peek_is(Syntax::RParen) {
            self.skip();
            return Ok(vec![]);
        }

        // parse (keyword: args)
        if self.peek2_is(Syntax::Colon) {
            return Ok(vec![self.keyword_args()?]);
        }

        let mut args = vec![];
        while let Some(tok) = self.peek() {
            match tok.kind {
                Syntax::RParen => {
                    self.skip();
                    break;
                }
                Syntax::Comma | Syntax::Semi => self.skip(),
                Syntax::LParen
                | Syntax::LCurly
                | Syntax::LStaple
                | Syntax::String(..)
                | Syntax::Number
                | Syntax::Word => {
                    args.push(self.expr()?);
                }
                _ => return self.error(")"),
            }
        }
        Ok(args)
    }

    // parse (keyword: args)
    fn keyword_args(&mut self) -> Result<Stmt> {
        let mut args = vec![];
        let mut is_kw = true;
        let mut keyword = String::new();
        while let Some(tok) = self.peek() {
            match tok.kind {
                Syntax::RParen => {
                    self.skip();
                    break;
                }
                Syntax::Comma | Syntax::Semi => self.skip(),
                Syntax::Word if is_kw => {
                    keyword = self.next().to_string();
                    self.expect(Syntax::Colon)?;
                    is_kw = false;
                }
                Syntax::LParen
                | Syntax::LCurly
                | Syntax::LStaple
                | Syntax::String(..)
                | Syntax::Number
                | Syntax::Word => {
                    args.push((std::mem::replace(&mut keyword, String::new()), self.expr()?));
                    is_kw = true;
                }
                _ => return self.error(")"),
            }
        }
        Ok(Stmt::Args(args))
    }

    /// Parse a block of code, either:
    /// - to the next Dedent if the next() char is an Indent
    ///   or
    /// - to the next ; if the next() char isn't an Indent
    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut block = vec![];

        // one line block via `do`
        if self.peek_is(Syntax::Word) && self.peek().filter(|p| p.to_str() == "do").is_some() {
            self.skip();
            return Ok(vec![self.stmt()?]);
        }

        self.expect(Syntax::Indent)?;
        while !self.peek_eof() {
            match self.peek_kind() {
                // keep going if we're indented
                Syntax::Semi => self.skip(),

                // pass these up the food chain
                Syntax::Dedent => break,

                // everything else is a stmt
                _ => block.push(self.stmt()?),
            };
        }
        self.expect(Syntax::Dedent)?;

        Ok(block)
    }

    /// Parse the innerHTML of a <tag>. Code and stuff.
    fn tag_body(&mut self) -> Result<Vec<Stmt>> {
        let mut block = vec![];
        let mut indented = false;

        if self.peek_is(Syntax::Indent) {
            self.skip();
            indented = true;
        }

        while !self.peek_eof() {
            match self.peek_kind() {
                // keep going if we're indented
                Syntax::Semi if indented => {
                    self.skip();
                }

                // pass these up the food chain
                Syntax::Dedent | Syntax::Semi => break,

                // Look for </closing> tag and bail if found.
                Syntax::LCaret if self.peek2_is(Syntax::Slash) => break,

                // everything else is a stmt
                _ => block.push(self.stmt()?),
            };
        }

        Ok(block)
    }

    /// Parse a single statement.
    fn stmt(&mut self) -> Result<Stmt> {
        match self.peek_kind() {
            // Literal
            Syntax::String(..)
            | Syntax::Number
            | Syntax::LParen
            | Syntax::LStaple
            | Syntax::LCurly => self.expr(),

            // Tag
            Syntax::LCaret => {
                // Otherwise parse as regular tag expression.
                self.expr()
            }

            // Keyword or Stmtession
            Syntax::Word => {
                let word = self.peek().unwrap();
                match word.literal() {
                    "if" => self.if_expr(),
                    "for" => self.for_expr(),
                    "def" => self.def_stmt(),
                    "return" => {
                        self.skip();
                        let ret = if self.peek_is(Syntax::Semi) {
                            Stmt::Return(bx!(Stmt::None))
                        } else {
                            Stmt::Return(bx!(self.expr()?))
                        };
                        self.expect(Syntax::Semi)?;
                        Ok(ret)
                    }
                    _ => {
                        // two words in a row become a string
                        if self
                            .peek2()
                            .filter(|p| {
                                matches!(p.kind, Syntax::Word | Syntax::Comma | Syntax::Colon)
                            })
                            .is_some()
                        {
                            let mut out = self.next().to_string();
                            while !self.peek_eof() {
                                match self.peek_kind() {
                                    Syntax::Word => out.push(' '),
                                    Syntax::Op | Syntax::Comma | Syntax::Colon => {}
                                    _ => break,
                                }
                                out.push_str(self.next().to_str())
                            }
                            Ok(out.into())
                        } else {
                            self.expr()
                        }
                    }
                }
            }

            // Unexpected
            _ => self.error("Stmt"),
        }
    }

    /// Parse a `for` statement:
    ///     for v in list
    ///     for k, v in map
    fn for_expr(&mut self) -> Result<Stmt> {
        self.expect(Syntax::Word)?; // for
        let mut key = None;
        let val;

        let word = self.expect(Syntax::Word)?.to_string();
        if self.peek_is(Syntax::Comma) {
            self.skip();
            key = Some(word);
            val = self.next().to_string();
        } else {
            val = word;
        }

        let in_word = self.expect(Syntax::Word)?;
        if in_word.literal() != "in" {
            return self.error("in");
        }

        let iter = self.expr()?;
        let body = self.block()?;

        Ok(Stmt::For(key, val, bx!(iter), body))
    }

    /// Parse a function definition.
    fn def_stmt(&mut self) -> Result<Stmt> {
        self.expect(Syntax::Word)?; // def
        let name = match self.peek_kind() {
            Syntax::Word | Syntax::Op => self.next(),
            _ => return self.error("function name"),
        }
        .to_string();

        let mut args = vec![];
        if self.peek_is(Syntax::LParen) {
            self.skip();
            self.eat(Syntax::Semi);
            while !self.peek_eof() && !self.peek_is(Syntax::RParen) {
                args.push(self.expect(Syntax::Word)?.to_string());
                if self.peek_is(Syntax::Comma) || self.peek_is(Syntax::Semi) {
                    self.next();
                } else {
                    break;
                }
            }
            self.eat(Syntax::Semi);
            self.expect(Syntax::RParen)?;
        }

        let body = self.block()?;
        Ok(Stmt::Assign(name, bx!(Stmt::Fn(args, body)), false))
    }

    /// Parse an if statement.
    fn if_expr(&mut self) -> Result<Stmt> {
        self.expect(Syntax::Word)?; // if
        let mut conds = vec![];
        let test = self.expr()?;
        let body = self.block()?;
        conds.push((test, body));
        while let Some(next) = self.peek() {
            if next.to_str() != "else" {
                break;
            }
            self.skip(); // skip else
            let mut test = Stmt::Bool(true);
            if let Some(word) = self.peek() {
                if word.literal() == "if" {
                    self.skip();
                    test = self.expr()?;
                }
            }
            let body = self.block()?;
            conds.push((test, body));
            continue;
        }
        Ok(Stmt::If(conds))
    }

    /// Parse a <tag> and its contents or a </tag>.
    fn tag(&mut self) -> Result<Stmt> {
        if self.peek2_is(Syntax::Slash) {
            self.close_tag()?;
            return Ok(Stmt::None);
        }

        let mut tag = self.open_tag()?;
        if tag.is_closed() {
            return Ok(Stmt::Tag(tag));
        }

        tag.set_body(self.tag_body()?);

        match self.peek_kind() {
            Syntax::Semi | Syntax::None => {
                if self.tags == 0 {
                    self.error("Open Tag")?;
                }
                self.tags -= 1;
            }
            Syntax::Dedent => {
                if self.tags == 0 {
                    self.error("Open Tag")?;
                }
                self.tags -= 1;
                self.skip();
            }
            _ => self.close_tag()?,
        }

        Ok(Stmt::Tag(tag))
    }

    /// Parse just a closing tag, starting after the <
    fn close_tag(&mut self) -> Result<()> {
        if self.tags == 0 {
            return self.error("Open Tag");
        }
        self.tags -= 1;
        self.expect(Syntax::LCaret)?;
        self.expect(Syntax::Slash)?;
        // </>
        if self.peek_is(Syntax::RCaret) {
            self.skip();
            return Ok(());
        }
        self.expect(Syntax::Word)?;
        self.expect(Syntax::RCaret)?;
        Ok(())
    }

    /// Parse a string <opening.tag with=attributes>
    /// starting after the <
    fn open_tag(&mut self) -> Result<Tag> {
        self.tags += 1;
        self.expect(Syntax::LCaret)?;
        let mut tag = Tag::new(match self.peek_kind() {
            Syntax::Op => Stmt::String("div".into()),
            _ => Stmt::String(self.expect(Syntax::Word)?.to_string()),
        });

        // <#shortcuts.only.work.in@the:first-part-of-the-tag gotcha=true/>
        let mut head = true;
        loop {
            let next = self.next();
            let pos = next.pos;
            match next.kind {
                Syntax::Semi => {}
                Syntax::RCaret => break,
                Syntax::Slash => {
                    tag.close();
                    self.tags -= 1;
                }
                Syntax::Op if head => match next.literal() {
                    "#" => {
                        let id = self.string()?;
                        if self.peek_is(Syntax::Equal) {
                            self.skip();
                            let cond = self.attr()?;
                            tag.set_id(Stmt::Call("when".into(), vec![cond, id]));
                        } else {
                            tag.set_id(id);
                        }
                    }
                    "." => {
                        let class = self.string()?;
                        if self.peek_is(Syntax::Equal) {
                            self.skip();
                            let cond = self.attr()?;
                            tag.add_class(Stmt::Call("when".into(), vec![cond, class]));
                        } else {
                            tag.add_class(class);
                        }
                    }
                    "@" | ":" => {
                        let attr_name = if next.literal() == "@" {
                            Stmt::String("name".into())
                        } else {
                            Stmt::String("type".into())
                        };
                        let expr = self.string()?;
                        if self.peek_is(Syntax::Equal) {
                            self.skip();
                            let cond = self.attr()?;
                            tag.add_attr(attr_name, Stmt::Call("when".into(), vec![cond, expr]));
                        } else {
                            tag.add_attr(attr_name.into(), expr);
                        }
                    }
                    _ => return self.error("# . @ or :"),
                },
                Syntax::Word | Syntax::String(true) => {
                    head = false;
                    self.back();
                    let name = self.string()?;
                    // single word attributes, like `defer`
                    if !self.peek_is(Syntax::Equal) {
                        tag.add_attr(name, Stmt::Bool(true));
                        continue;
                    }
                    self.expect(Syntax::Equal)?;
                    match self.peek_kind() {
                        Syntax::Number | Syntax::String(..) => tag.add_attr(name, self.atom()?),
                        Syntax::Word => tag.add_attr(
                            name,
                            if self.peek().filter(|p| p.to_str().contains('{')).is_some() {
                                self.string()?
                            } else {
                                self.expr()?
                            },
                        ),
                        Syntax::JS => tag.add_attr(
                            name,
                            Stmt::String(format!(
                                "(function(e){{ {} }})(event);",
                                self.next().to_string()
                            )),
                        ),

                        _ => return self.error("Word, Number, or String"),
                    }
                }
                _ => return pos_error!(pos, "Expected Attribute or >, got {:?}", next),
            }
        }

        Ok(tag)
    }

    pub fn attr(&mut self) -> Result<Stmt> {
        if let Some(tok) = self.peek() {
            match tok.kind {
                Syntax::String(..) => self.string(),
                Syntax::Word => {
                    let lit = tok.literal();
                    if lit.starts_with('{') && lit.ends_with('}') {
                        self.string()
                    } else if tok.literal().contains('{') {
                        self.string()
                    } else {
                        self.word()
                    }
                }
                _ => self.error("String or Word"),
            }
        } else {
            Ok(Stmt::None)
        }
    }
}
