use {
    crate::{scan, Error, Expr, Result, Syntax, Tag, Token, AST},
    std::collections::HashMap,
};

#[cfg(debug_assertions)]
const STACK_SIZE: usize = 1000; // infinite loop protection

#[derive(Debug)]
pub struct Parser<'s, 't> {
    ast: AST,                           // what we're building
    tokens: &'t [Token<'s>],            // code
    pos: usize,                         // position in tokens vec
    tags: usize,                        // open tags
    operators: HashMap<String, String>, // operators like + - * /

    #[cfg(debug_assertions)]
    peeked: usize, // infinite loop protection hack
}

pub fn parse<'t>(tokens: &'t [Token]) -> Result<AST> {
    let mut parser = Parser::from(tokens);
    parser.parse()?;
    Ok(parser.ast)
}

impl<'s, 't> Parser<'s, 't> {
    /// Create a `Parser` from a `TokenStream`.
    pub fn from(tokens: &'t [Token<'s>]) -> Parser<'s, 't> {
        Parser {
            tokens,
            ast: AST::new(),
            tags: 0,
            pos: 0,
            operators: default_operators(),
            #[cfg(debug_assertions)]
            peeked: 0,
        }
    }

    /// Parse `TokenStream` into `AST`.
    pub fn parse(&mut self) -> Result<()> {
        let mut ast = AST::new();

        while !self.peek_eof() {
            let mut block = self.block()?;
            ast.exprs.append(&mut block);
            match self.peek_kind() {
                Syntax::Dedent | Syntax::Special(';') => self.skip(),
                _ => {}
            }
        }

        self.ast = ast;
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
        Err(if let Some(got) = self.try_next() {
            Error::new(
                format!("expected {}, got {:?}", msg.as_ref(), got.kind),
                got.pos,
                got.len,
            )
        } else {
            Error::new(format!("expected {}, got EOF", msg.as_ref()), 0, 0)
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

    /// Parse a bool.
    fn boolean(&mut self) -> Result<Expr> {
        Ok(Expr::Bool(self.expect(Syntax::Bool)?.literal() == "true"))
    }

    /// Parse a number.
    fn number(&mut self) -> Result<Expr> {
        Ok(Expr::Number(self.expect(Syntax::Number)?.to_usize()?))
    }

    /// Parse a string.
    fn string(&mut self) -> Result<Expr> {
        let tok = self.next();
        let is_interpolated = match tok.kind {
            Syntax::String(is) => is,
            _ => return self.error("String"),
        };

        let lit = tok.to_string();
        if is_interpolated && lit.contains('{') {
            let mut parts = vec![];
            let mut idx = 0;
            while let Some(i) = lit[idx..].find('{') {
                // check for escaped \{}
                if i > 0 && lit[idx..].bytes().nth(i - 1).unwrap_or(b'0') == b'\\' {
                    parts.push(Expr::String(lit[idx..i + idx - 1].into()));
                    parts.push(Expr::String(lit[idx + i..i + idx + 1].into()));
                    idx += i + 1;
                    continue;
                }

                {
                    let s = &lit[idx..i + idx];
                    if !s.is_empty() {
                        parts.push(Expr::String(s.into()));
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
                let mut ast = scan(&lit[idx..end])
                    .and_then(|t| parse(&t))
                    .map_err(|mut e| {
                        e.pos += tok.pos + idx - 1; // probably not right yet...
                        e
                    })?;
                parts.append(&mut ast.exprs);
                idx = end + 1;
            }
            if idx < lit.len() {
                parts.push(Expr::String(lit[idx..].into()));
            }
            if parts.len() == 1 {
                Ok(parts.remove(0))
            } else {
                Ok(Expr::Call("concat".into(), parts))
            }
        } else {
            Ok(Expr::String(lit))
        }
    }

    /// Parse a word.
    fn word(&mut self) -> Result<Expr> {
        let word = self.expect(Syntax::Word)?;
        Ok(match word.literal() {
            "true" | "false" => Expr::Bool(word.literal() == "true"),
            _ => Expr::Word(word.to_string()),
        })
    }

    /// Parse a code expression.
    fn expr(&mut self) -> Result<Expr> {
        if let Some(p) = self.peek2() {
            if p.kind == Syntax::Word && matches!(p.literal(), ":=" | "=") {
                let reassign = p.literal() == "=";
                let name = self.expect(Syntax::Word)?.to_string();
                self.skip(); // skip op
                return Ok(Expr::Assign(name, Box::new(self.expr()?), reassign));
            }
        }

        let left = self.atom()?;
        if let Some(next) = self.peek() {
            if next.kind == Syntax::Word {
                let lit = next.to_string();
                if lit == "." {
                    // convert word to str, ex: map.key => index(map, "key")
                    self.skip();
                    let right = self.expr()?;
                    if let Expr::Word(word) = right {
                        return Ok(Expr::Call("index".into(), vec![left, Expr::String(word)]));
                    } else {
                        return Ok(Expr::Call("index".into(), vec![left, right]));
                    }
                } else if let Some(f) = self.operators.get(&lit) {
                    let op = f.clone();
                    self.skip();
                    let right = self.expr()?;
                    return Ok(Expr::Call(op, vec![left, right]));
                }
            }
        }
        Ok(left)
    }

    /// Parse an indivisible unit, as the Ancient Greeks would say.
    fn atom(&mut self) -> Result<Expr> {
        match self.peek_kind() {
            Syntax::String(..) => Ok(self.string()?),
            Syntax::Number => Ok(self.number()?),
            Syntax::Bool => Ok(self.boolean()?),
            Syntax::Bracket('(') => {
                self.skip();
                let expr = self.expr()?;
                self.expect(Syntax::Bracket(')'))?;
                Ok(expr)
            }
            // List
            Syntax::Bracket('[') => {
                self.skip();
                self.eat(Syntax::Special(';'));
                let mut list = vec![];
                while !self.peek_eof() && !self.peek_is(Syntax::Bracket(']')) {
                    list.push(self.expr()?);
                    if self.peek_is(Syntax::Special(';')) {
                        self.skip();
                    } else if self.peek_is(Syntax::Bracket(']')) {
                        break;
                    } else {
                        self.expect(Syntax::Special(','))?;
                    }
                }
                self.eat(Syntax::Special(';'));
                self.expect(Syntax::Bracket(']'))?;
                Ok(Expr::List(list))
            }
            // Map
            Syntax::Bracket('{') => {
                self.skip();
                self.eat(Syntax::Special(';'));
                let mut map = vec![];
                while !self.peek_eof() && !self.peek_is(Syntax::Bracket('}')) {
                    let key = match self.peek_kind() {
                        Syntax::Word | Syntax::String(..) | Syntax::Number => {
                            self.next().to_string()
                        }
                        _ => return self.error("String key name"),
                    };
                    self.expect(Syntax::Special(':'))?;
                    let val = self.expr()?;
                    map.push((key, val));
                    if self.peek_is(Syntax::Special(';')) {
                        self.skip();
                    } else if self.peek_is(Syntax::Bracket('}')) {
                        break;
                    } else {
                        self.expect(Syntax::Special(','))?;
                    }
                }
                self.eat(Syntax::Special(';'));
                self.expect(Syntax::Bracket('}'))?;
                Ok(Expr::Map(map))
            }
            Syntax::Word => {
                let word = self.word()?;
                if !self.peek_is(Syntax::Bracket('(')) {
                    return Ok(word);
                } else {
                    self.expect(Syntax::Bracket('('))?;
                    let name = word.to_string();
                    let mut args = vec![];
                    while let Some(tok) = self.peek() {
                        match tok.kind {
                            Syntax::Bracket(')') => {
                                self.skip();
                                break;
                            }
                            Syntax::Special(',') => self.skip(),
                            Syntax::String(..) | Syntax::Number | Syntax::Bool | Syntax::Word => {
                                args.push(self.expr()?);
                            }
                            _ => return self.error(")"),
                        }
                    }
                    Ok(Expr::Call(name, args))
                }
            }
            _ => self.error("Atom"),
        }
    }

    /// Parse a block of code, either:
    /// - to the next Dedent if the next() char is an Indent
    ///   or
    /// - to the next ; if the next() char isn't an Indent
    fn block(&mut self) -> Result<Vec<Expr>> {
        let mut block = vec![];
        let mut indented = false;

        if self.peek_is(Syntax::Indent) {
            self.skip();
            indented = true;
        }

        while !self.peek_eof() {
            match self.peek_kind() {
                // Tag
                Syntax::Bracket('<') => {
                    if !indented
                        && self
                            .peek2()
                            .filter(|p| p.kind == Syntax::Special('/'))
                            .is_some()
                    {
                        break;
                    }
                    block.push(self.tag()?);
                }

                // Literal
                Syntax::String(..)
                | Syntax::Number
                | Syntax::Bracket('(')
                | Syntax::Bracket('[')
                | Syntax::Bracket('{') => {
                    block.push(self.expr()?);
                }

                // Expression
                Syntax::Word => {
                    if let Some(word) = self.peek() {
                        match word.literal() {
                            "if" => block.push(self.if_expr()?),
                            "for" => block.push(self.for_expr()?),
                            "op!" => {
                                self.skip();
                                let op = self.expect(Syntax::Word)?.to_string();
                                let f = self.expect(Syntax::Word)?.to_string();
                                self.operators.insert(op, f);
                            }
                            _ => block.push(self.expr()?),
                        }
                    }
                }

                // keep going if we're indented
                Syntax::Special(';') if indented => {
                    self.skip();
                }

                // pass these up the food chain
                Syntax::Dedent | Syntax::Special(';') => break,

                // probably implicit text...
                Syntax::Special(c) => {
                    self.skip();
                    block.push(Expr::Word(c.to_string()));
                }

                // Unexpected
                _ => return self.error("Block stmt"),
            };
        }

        Ok(block)
    }

    /// Parse a `for` statement:
    ///     for v in list
    ///     for k, v in map
    fn for_expr(&mut self) -> Result<Expr> {
        self.expect(Syntax::Word)?; // for
        let mut key = None;
        let val;

        let word = self.expect(Syntax::Word)?.to_string();
        if self.peek_is(Syntax::Special(',')) {
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

        self.expect(Syntax::Dedent)?;
        Ok(Expr::For(key, val, Box::new(iter), body))
    }

    /// Parse an if statement.
    fn if_expr(&mut self) -> Result<Expr> {
        self.expect(Syntax::Word)?; // if
        let mut conds = vec![];
        let test = self.expr()?;
        let body = self.block()?;
        conds.push((test, body));
        while self.peek_is(Syntax::Dedent) {
            if let Some(next) = self.peek2() {
                if next.literal() == "else" {
                    self.skip(); // skip dedent
                    self.skip(); // skip else
                    let mut test = Expr::Bool(true);
                    if let Some(word) = self.peek() {
                        if word.literal() == "if" {
                            self.skip();
                            test = self.expr()?;
                        }
                    }
                    let body = if self.peek_is(Syntax::Indent) {
                        self.block()?
                    } else {
                        vec![self.expr()?]
                    };
                    conds.push((test, body));
                    continue;
                }
            }
            break;
        }
        self.expect(Syntax::Dedent)?;
        Ok(Expr::If(conds))
    }

    /// Parse a <tag> and its contents or a </tag>.
    fn tag(&mut self) -> Result<Expr> {
        if self
            .peek2()
            .filter(|p| p.kind == Syntax::Special('/'))
            .is_some()
        {
            self.close_tag()?;
            return Ok(Expr::None);
        }

        let mut tag = self.open_tag()?;
        if tag.is_closed() {
            return Ok(Expr::Tag(tag));
        }

        tag.set_body(self.block()?);

        match self.peek_kind() {
            Syntax::Special(';') | Syntax::None => self.tags -= 1,
            Syntax::Dedent => {
                self.tags -= 1;
                self.skip();
            }
            _ => self.close_tag()?,
        }

        Ok(Expr::Tag(tag))
    }

    /// Parse just a closing tag, starting after the <
    fn close_tag(&mut self) -> Result<()> {
        if self.tags == 0 {
            return self.error("open tags");
        }
        self.tags -= 1;
        self.expect(Syntax::Bracket('<'))?;
        self.expect(Syntax::Special('/'))?;
        // </>
        if self.peek_is(Syntax::Bracket('>')) {
            self.skip();
            return Ok(());
        }
        self.expect(Syntax::Word)?;
        self.expect(Syntax::Bracket('>'))?;
        Ok(())
    }

    /// Parse a string <opening.tag with=attributes>
    /// starting after the <
    fn open_tag(&mut self) -> Result<Tag> {
        self.tags += 1;
        self.expect(Syntax::Bracket('<'))?;
        let mut tag = Tag::new(match self.peek_kind() {
            Syntax::Special(_) => Expr::String("div".into()),
            _ => self.attr()?,
        });

        loop {
            let next = self.next();
            let pos = next.pos;
            match next.kind {
                Syntax::Special(';') => {}
                Syntax::Bracket('>') => break,
                Syntax::Special('/') => {
                    tag.close();
                    self.tags -= 1;
                }
                Syntax::Special('#') => {
                    let id = self.attr()?;
                    if self.peek_is(Syntax::Special('=')) {
                        self.next();
                        let cond = self.expr()?;
                        tag.set_id(Expr::Call("when".into(), vec![cond, id]));
                    } else {
                        tag.set_id(id);
                    }
                }
                Syntax::Special('.') => {
                    let class = self.attr()?;
                    if self.peek_is(Syntax::Special('=')) {
                        self.next();
                        let cond = self.expr()?;
                        tag.add_class(Expr::Call("when".into(), vec![cond, class]));
                    } else {
                        tag.add_class(class);
                    }
                }
                Syntax::Special('@') | Syntax::Special(':') => {
                    let attr_name = if let Syntax::Special('@') = &next.kind {
                        Expr::String("name".into())
                    } else {
                        Expr::String("type".into())
                    };
                    let expr = self.attr()?;
                    if self.peek_is(Syntax::Special('=')) {
                        self.next();
                        let cond = self.expr()?;
                        tag.add_attr(attr_name, Expr::Call("when".into(), vec![cond, expr]));
                    } else {
                        tag.add_attr(attr_name.into(), expr);
                    }
                }
                Syntax::String(true) => {
                    self.back();
                    let name = self.attr()?;
                    self.expect(Syntax::Special('='))?;
                    match self.peek_kind() {
                        Syntax::Number | Syntax::String(..) | Syntax::Word => {
                            tag.add_attr(name, self.atom()?)
                        }
                        Syntax::JS => tag.add_attr(
                            name,
                            Expr::String(format!(
                                "(function(e){{ {} }})(event);return false;",
                                self.next().to_string()
                            )),
                        ),

                        _ => return pos_error!(pos, "Expected Word, Number, or String"),
                    }
                }
                _ => return pos_error!(pos, "Expected Attribute or >, got {:?}", next),
            }
        }

        Ok(tag)
    }

    /// Parse a tag attribute, which may have {interpolation}.
    fn attr(&mut self) -> Result<Expr> {
        self.string()
    }
}

/// Probably a better way to do this, but it works for now and lets us
/// define new operators in Hatter source code using `op!`.
fn default_operators() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("==".into(), "eq".into());
    map.insert("!=".into(), "neq".into());
    map.insert("+".into(), "add".into());
    map.insert("-".into(), "sub".into());
    map.insert("*".into(), "mul".into());
    map.insert("/".into(), "div".into());
    map.insert(">".into(), "gt".into());
    map.insert(">=".into(), "gte".into());
    map.insert("<".into(), "lt".into());
    map.insert("<=".into(), "lte".into());
    map.insert(".".into(), "index".into());
    map
}
