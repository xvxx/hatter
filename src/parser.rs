use crate::{Error, Expr, Result, Syntax, Tag, TokenPos, TokenStream, AST};

const STACK_SIZE: usize = 1000; // infinite loop protection

#[derive(Debug)]
pub struct Parser {
    ast: AST,
    tokens: TokenStream,
    pos: usize,
    indent: usize,
    tags: usize,   // open tags
    peeked: usize, // infinite loop protection hack
}

pub fn parse(tokens: TokenStream) -> Result<AST> {
    let mut parser = Parser::from(tokens);
    parser.parse()?;
    Ok(parser.ast)
}

impl Parser {
    /// Create a `Parser` from a `TokenStream`.
    pub fn from(tokens: TokenStream) -> Parser {
        Parser {
            tokens,
            pos: 0,
            ast: AST::new(),
            peeked: 0,
            indent: 0,
            tags: 0,
        }
    }

    /// Peek at next `Token`.
    fn peek(&mut self) -> Option<TokenPos> {
        self.peeked += 1;
        if self.peeked > STACK_SIZE {
            panic!("infinite loop while peek()ing: {:?}", self.tokens.peek());
        }
        self.tokens.peek()
    }

    /// Peek two ahead.
    fn peek2(&mut self) -> Option<TokenPos> {
        self.peeked += 1;
        if self.peeked > STACK_SIZE {
            panic!("infinite loop while peek()ing: {:?}", self.tokens.peek());
        }
        self.tokens.peek2()
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
    fn try_next(&mut self) -> Option<TokenPos> {
        self.tokens.next()
    }

    /// Advance iterator an return next `Token`.
    fn next(&mut self) -> TokenPos {
        self.peeked = 0;
        self.tokens.next().unwrap()
    }

    /// Return current `Token`.
    fn current(&mut self) -> TokenPos {
        self.tokens.current().unwrap()
    }

    /// Trigger parse error for next() token.
    fn error<S: AsRef<str>>(&mut self, msg: S) -> Error {
        if let Some(got) = self.try_next() {
            Error::new(
                format!("expected {}, got {:?}", msg.as_ref(), got.kind),
                got.pos,
                got.len,
            )
        } else {
            Error::new(format!("expected {}, got EOF", msg.as_ref()), 0, 0)
        }
    }

    /// Trigger parse error for current() token.
    fn error_current<S: AsRef<str>>(&mut self, msg: S) -> Error {
        let got = self.current();
        Error::new(
            format!("expected {}, got {:?}", msg.as_ref(), got.kind),
            got.pos,
            got.len,
        )
    }

    /// Consumes and returns the next token if it's of `kind`,
    /// otherwise errors.
    fn expect(&mut self, kind: Syntax) -> Result<TokenPos> {
        if self.peek_kind() == kind {
            Ok(self.next())
        } else {
            Err(self.error(format!("{:?}", kind)))
        }
    }

    /// Parse `TokenStream` into `AST`.
    pub fn parse(&mut self) -> Result<()> {
        let mut ast = AST::new();

        while let Some(tok) = self.peek() {
            let node = match tok.kind {
                // Tag
                Syntax::Bracket('<') => self.tag()?,

                // Syntax
                Syntax::Special(';') => {
                    self.next();
                    continue;
                }

                // Unexpected
                _ => return Err(self.error("HTML Tag")),
            };

            ast.exprs.push(node);
        }

        self.ast = ast;
        Ok(())
    }

    /// Parse a literal as a string expression.
    fn as_string(&mut self) -> Result<Expr> {
        if let Some(next) = self.try_next() {
            Ok(Expr::String(next.to_string()))
        } else {
            Err(self.error("a literal"))
        }
    }

    /// Parse a bool.
    fn boolean(&mut self) -> Result<Expr> {
        Ok(Expr::Bool(self.expect(Syntax::Bool)?.literal() == "true"))
    }

    /// Parse a number.
    fn number(&mut self) -> Result<Expr> {
        Ok(Expr::Number(self.expect(Syntax::Number)?.to_f64()?))
    }

    /// Parse a string.
    fn string(&mut self) -> Result<Expr> {
        Ok(Expr::String(self.expect(Syntax::String)?.to_string()))
    }

    /// Parse a word.
    fn word(&mut self) -> Result<Expr> {
        Ok(Expr::Word(self.expect(Syntax::Word)?.to_string()))
    }

    /// Parse a code expression.
    fn expr(&mut self) -> Result<Expr> {
        match self.peek_kind() {
            Syntax::String => Ok(self.string()?),
            Syntax::Number => Ok(self.number()?),
            Syntax::Bool => Ok(self.boolean()?),
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
                                self.next();
                                break;
                            }
                            Syntax::Special(',') => {
                                self.next();
                            }
                            Syntax::String | Syntax::Number | Syntax::Bool | Syntax::Word => {
                                args.push(self.expr()?);
                            }
                            _ => return Err(self.error(")")),
                        }
                    }
                    Ok(Expr::Call(name, args))
                }
            }
            _ => Err(self.error("Expression")),
        }
    }

    /// Parse the content of a <tag>CONTENT</tag>.
    fn content(&mut self) -> Result<Vec<Expr>> {
        let mut block = vec![];
        let mut indented = false;

        if self.peek_is(Syntax::Indent) {
            self.next();
            indented = true;
        }

        loop {
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
                Syntax::String | Syntax::Number => {
                    block.push(self.as_string()?);
                }

                // Expression
                Syntax::Word => {
                    let expr = self.expr()?;
                    if let Expr::Word(word) = &expr {
                        match word.as_ref() {
                            "if" => {
                                block.push(self.if_expr()?);
                                continue;
                            }
                            "for" => {
                                block.push(self.for_expr()?);
                                continue;
                            }
                            _ => {}
                        }
                    }
                    block.push(expr);
                }

                // keep going if we're indented
                Syntax::Special(';') if indented => {
                    self.next();
                }

                // pass these up the food chain
                Syntax::Dedent | Syntax::Special(';') => break,

                // probably implicit text...
                Syntax::Special(c) => {
                    self.next();
                    block.push(Expr::Word(c.to_string()));
                }

                // Unexpected
                _ => return Err(self.error("Tag contents")),
            };
        }

        Ok(block)
    }

    /// Parse a `for` statement:
    ///     for v in list
    ///     for k, v in map
    fn for_expr(&mut self) -> Result<Expr> {
        let mut key = None;
        let val;

        let word = self.expect(Syntax::Word)?.to_string();
        if self.peek_is(Syntax::Special(',')) {
            self.next();
            key = Some(word);
            val = self.next().to_string();
        } else {
            val = word;
        }

        let in_word = self.expect(Syntax::Word)?;
        if in_word.literal() != "in" {
            return Err(self.error("in"));
        }

        let iter = self.expr()?;
        let body = if self.peek_is(Syntax::Indent) {
            self.content()?
        } else {
            vec![self.expr()?]
        };

        self.expect(Syntax::Dedent)?;
        Ok(Expr::For(key, val, Box::new(iter), body))
    }

    /// Parse an if statement.
    fn if_expr(&mut self) -> Result<Expr> {
        let mut conds = vec![];
        let test = self.expr()?;
        let body = if self.peek_is(Syntax::Indent) {
            self.content()?
        } else {
            vec![self.expr()?]
        };
        conds.push((test, body));
        if self.peek_is(Syntax::Dedent) {
            if let Some(next) = self.peek2() {
                if next.literal() == "else" {
                    self.next();
                    self.next();
                    let body = if self.peek_is(Syntax::Indent) {
                        self.content()?
                    } else {
                        vec![self.expr()?]
                    };
                    conds.push((Expr::Bool(true), body));
                }
            }
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

        tag.contents = self.content()?;

        match self.peek_kind() {
            Syntax::Special(';') => self.tags -= 1,
            Syntax::Dedent => {
                self.tags -= 1;
                self.next();
            }
            _ => self.close_tag()?,
        }

        Ok(Expr::Tag(tag))
    }

    /// Parse just a closing tag, starting after the <
    fn close_tag(&mut self) -> Result<()> {
        if self.tags == 0 {
            return Err(self.error("open tags"));
        }
        self.tags -= 1;
        self.expect(Syntax::Bracket('<'))?;
        self.expect(Syntax::Special('/'))?;
        // </>
        if self.peek_is(Syntax::Bracket('>')) {
            self.next();
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
            Syntax::Special(_) => "div".to_string(),
            _ => self.expect(Syntax::Word)?.to_string(),
        });

        loop {
            let next = self.next();
            match next.kind {
                Syntax::Bracket('>') => break,
                Syntax::Special('/') => {
                    tag.close();
                    self.tags -= 1;
                }
                Syntax::Special('#') => tag.add_attr("id", self.expect(Syntax::Word)?.literal()),
                Syntax::Special('.') => tag.add_class(self.expect(Syntax::Word)?.to_string()),
                Syntax::Special('@') => tag.add_attr("name", self.expect(Syntax::Word)?.literal()),
                Syntax::Special(':') => tag.add_attr("type", self.expect(Syntax::Word)?.literal()),
                Syntax::Word => {
                    let name = next.to_string();
                    self.expect(Syntax::Special('='))?;
                    match self.peek_kind() {
                        Syntax::Number | Syntax::String | Syntax::Word => {
                            tag.add_attr(name, self.next().to_string())
                        }
                        Syntax::JS => tag.add_attr(
                            name,
                            format!(
                                "(function(e){{ {} }})(event);return false;",
                                self.next().to_string()
                            ),
                        ),

                        _ => return Err(self.error_current("Word, Number, or String")),
                    }
                }
                _ => return Err(self.error_current("Attribute or >")),
            }
        }

        Ok(tag)
    }
}
