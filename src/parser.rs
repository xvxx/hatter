use crate::{
    token::{TokenKind, TokenPos, TokenStream},
    Error, Result, Stmt, Tag, AST,
};

const STACK_SIZE: usize = 1000; // infinite loop protection

pub struct Parser {
    ast: AST,
    tokens: TokenStream,
    pos: usize,
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

    /// Parse the content of a <tag>CONTENT</tag>.
    fn content(&mut self) -> Result<Stmt> {
        loop {
            match self.peek_kind() {
                // Tag
                TokenKind::Bracket('<') => self.tag()?,

                // Unexpected
                _ => return Err(self.error("HTML Tag")),
            };
        }
    }

    /// Parse a single code statment (IF, FOR, etc)
    fn code(&mut self) -> Result<Stmt> {
        Ok(Stmt::Expr)
    }

    /// Parse a <tag> and its contents or a </tag>.
    fn tag(&mut self) -> Result<Stmt> {
        self.expect(TokenKind::Bracket('<'))?;

        if self.peek_kind() == TokenKind::Special('/') {
            self.close_tag()?;
            return Ok(Stmt::None);
        }

        let tag = self.open_tag()?;
        if tag.is_closed() {
            return Ok(Stmt::Tag(tag));
        }
        // tag.contents = self.content()?;
        self.close_tag()?;
        Ok(Stmt::Tag(tag))
    }

    /// Parse just a closing tag, starting after the <
    fn close_tag(&mut self) -> Result<()> {
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
        let mut tag = Tag::new(self.expect(TokenKind::Word)?.to_string());

        loop {
            let next = self.next();
            match next.kind {
                TokenKind::Bracket('>') => break,
                TokenKind::Bracket('/') => {
                    tag.close();
                    self.tags -= 1;
                }
                TokenKind::Special('#') => tag.id = Some(self.expect(TokenKind::Word)?.to_string()),
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
