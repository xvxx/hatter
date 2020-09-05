use crate::{
    token::{TokenKind, TokenPos, TokenStream},
    Error, Expr, Result, Stmt, Tag, AST,
};

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
    fn peek_kind(&mut self) -> TokenKind {
        self.peek().map(|t| t.kind).unwrap_or(TokenKind::None)
    }

    /// Check the next token's kind.
    fn peek_is(&mut self, kind: TokenKind) -> bool {
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

    /// Trigger parse error for next() token.
    fn error<S: AsRef<str>>(&mut self, msg: S) -> Error {
        println!("{:#?}", self.ast);
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

    /// Consumes and returns the next token if it's of `kind`,
    /// otherwise errors.
    fn expect(&mut self, kind: TokenKind) -> Result<TokenPos> {
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
                TokenKind::Bracket('<') => self.tag()?,

                // Unexpected
                _ => return Err(self.error("HTML Tag")),
            };

            ast.stmts.push(node);
        }

        self.ast = ast;
        Ok(())
    }

    /// Parse a literal as a string.
    fn as_string(&mut self) -> Result<Stmt> {
        if let Some(next) = self.try_next() {
            Ok(Stmt::Expr(Expr::String(next.to_string())))
        } else {
            Err(self.error("a literal"))
        }
    }

    /// Parse a string.
    fn string(&mut self) -> Result<Stmt> {
        Ok(Stmt::Expr(Expr::String(
            self.expect(TokenKind::String)?.to_string(),
        )))
    }

    /// Parse a word.
    fn word(&mut self) -> Result<Stmt> {
        Ok(Stmt::Expr(Expr::Word(
            self.expect(TokenKind::Word)?.to_string(),
        )))
    }

    /// Parse the content of a <tag>CONTENT</tag>.
    fn content(&mut self) -> Result<Vec<Stmt>> {
        let mut block = vec![];
        let mut indented = false;

        if self.peek_kind() == TokenKind::Indent {
            self.next();
            indented = true;
        }

        loop {
            match self.peek_kind() {
                // Tag
                TokenKind::Bracket('<') => {
                    if !indented
                        && self
                            .peek2()
                            .filter(|p| p.kind == TokenKind::Special('/'))
                            .is_some()
                    {
                        break;
                    }
                    block.push(self.tag()?);
                }

                // Literal
                TokenKind::String | TokenKind::Number | TokenKind::Word => {
                    block.push(self.as_string()?);
                }

                // keep going if we're indented
                TokenKind::Special(';') if indented => {
                    self.next();
                }

                // pass these up the food chain
                TokenKind::Dedent | TokenKind::Special(';') => break,

                // Treat as literals for now
                TokenKind::Special(_) => block.push(self.as_string()?),

                // Unexpected
                _ => return Err(self.error("Tag contents")),
            };
        }

        Ok(block)
    }

    /// Parse a <tag> and its contents or a </tag>.
    fn tag(&mut self) -> Result<Stmt> {
        if self
            .peek2()
            .filter(|p| p.kind == TokenKind::Special('/'))
            .is_some()
        {
            self.close_tag()?;
            return Ok(Stmt::None);
        }

        let mut tag = self.open_tag()?;
        if tag.is_closed() {
            return Ok(Stmt::Tag(tag));
        }

        tag.contents = self.content()?;

        match self.peek_kind() {
            TokenKind::Special(';') => self.tags -= 1,
            TokenKind::Dedent => {
                self.tags -= 1;
                self.next();
            }
            _ => self.close_tag()?,
        }

        Ok(Stmt::Tag(tag))
    }

    /// Parse just a closing tag, starting after the <
    fn close_tag(&mut self) -> Result<()> {
        if self.tags == 0 {
            return Err(self.error("open tags"));
        }
        self.tags -= 1;
        self.expect(TokenKind::Bracket('<'))?;
        self.expect(TokenKind::Special('/'))?;
        // </>
        if self.peek_kind() == TokenKind::Bracket('>') {
            self.next();
            return Ok(());
        }
        self.expect(TokenKind::Word)?;
        self.expect(TokenKind::Bracket('>'))?;
        Ok(())
    }

    /// Parse a string <opening.tag with=attributes>
    /// starting after the <
    fn open_tag(&mut self) -> Result<Tag> {
        self.tags += 1;
        self.expect(TokenKind::Bracket('<'))?;
        let mut tag = Tag::new(self.expect(TokenKind::Word)?.to_string());

        loop {
            let next = self.next();
            match next.kind {
                TokenKind::Bracket('>') => break,
                TokenKind::Bracket('/') => {
                    tag.close();
                    self.tags -= 1;
                }
                TokenKind::Special('#') => {
                    tag.add_attr("id", self.expect(TokenKind::Word)?.literal())
                }
                TokenKind::Special('.') => tag.add_class(self.expect(TokenKind::Word)?.to_string()),
                TokenKind::Special('@') => {
                    tag.add_attr("name", self.expect(TokenKind::Word)?.literal())
                }
                TokenKind::Special(':') => {
                    tag.add_attr("type", self.expect(TokenKind::Word)?.literal())
                }
                TokenKind::Word => {
                    let name = next.to_string();
                    self.expect(TokenKind::Special('='))?;
                    match self.peek_kind() {
                        TokenKind::Number | TokenKind::String | TokenKind::Word => {
                            tag.add_attr(name, self.next().to_string())
                        }
                        _ => return Err(self.error("Word, Number, or String")),
                    }
                }
                _ => return Err(self.error("Attribute or >")),
            }
        }

        Ok(tag)
    }
}
