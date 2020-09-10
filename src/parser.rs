use {
    crate::{Error, Expr, Result, Syntax, Tag, TokenPos, TokenStream, AST},
    std::collections::HashMap,
};

const STACK_SIZE: usize = 1000; // infinite loop protection

#[derive(Debug)]
pub struct Parser {
    ast: AST,                           // what we're building
    tokens: TokenStream,                // code
    tags: usize,                        // open tags
    operators: HashMap<String, String>, // operators like + - * /

    #[cfg(debug_assertions)]
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
            ast: AST::new(),
            tags: 0,
            operators: default_operators(),
            #[cfg(debug_assertions)]
            peeked: 0,
        }
    }

    /// Parse `TokenStream` into `AST`.
    pub fn parse(&mut self) -> Result<()> {
        let mut ast = AST::new();
        let mut first = true;
        let mut autohtml = false;

        while !self.peek_eof() {
            let mut block = self.block()?;
            if first {
                first = false;
                if first_is_head(&block) {
                    autohtml = true;
                    ast.exprs.push(Expr::String("<!DOCTYPE html>".into()));
                    ast.exprs.push(Expr::String("<html>".into()));
                }
            }
            ast.exprs.append(&mut block);
            match self.peek_kind() {
                Syntax::Dedent | Syntax::Special(';') => {
                    self.next();
                }
                _ => {}
            }
        }

        if autohtml {
            ast.exprs.push(Expr::String("</html>".into()));
        }

        self.ast = ast;
        Ok(())
    }

    /// Peek at next `Token`.
    fn peek(&mut self) -> Option<TokenPos> {
        #[cfg(debug_assertions)]
        {
            self.peeked += 1;
            if self.peeked > STACK_SIZE {
                panic!("infinite loop while peek()ing: {:?}", self.tokens.peek());
            }
        }
        self.tokens.peek()
    }

    /// Peek two ahead.
    fn peek2(&mut self) -> Option<TokenPos> {
        #[cfg(debug_assertions)]
        {
            self.peeked += 1;
            if self.peeked > STACK_SIZE {
                panic!("infinite loop while peek()ing: {:?}", self.tokens.peek());
            }
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

    /// Check the next token's literal value.
    fn peek_lit_is(&mut self, lit: &str) -> bool {
        if let Some(p) = self.peek() {
            p.literal() == lit
        } else {
            false
        }
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
        #[cfg(debug_assertions)]
        {
            self.peeked = 0;
        }
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

    /// Parse a """triple string""".
    fn triple_string(&mut self) -> Result<Expr> {
        Ok(Expr::String(self.expect(Syntax::TripleString)?.to_string()))
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
        let left = self.atom()?;
        if let Some(next) = self.peek() {
            if next.kind == Syntax::Word {
                let lit = next.to_string();
                if lit == ":=" || lit == "=" {
                    self.next();
                    return Ok(Expr::Assign(
                        left.to_string(),
                        Box::new(self.expr()?),
                        lit == "=",
                    ));
                } else if let Some(f) = self.operators.get(&lit) {
                    let op = f.clone();
                    self.next();
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
            Syntax::String => Ok(self.string()?),
            Syntax::TripleString => Ok(self.triple_string()?),
            Syntax::Number => Ok(self.number()?),
            Syntax::Bool => Ok(self.boolean()?),
            Syntax::Bracket('(') => {
                self.next();
                let expr = self.expr()?;
                self.expect(Syntax::Bracket(')'))?;
                Ok(expr)
            }
            // List
            Syntax::Bracket('[') => {
                self.next();
                let mut list = vec![];
                while !self.peek_eof() {
                    list.push(self.expr()?);
                    if self.peek_is(Syntax::Bracket(']')) {
                        break;
                    } else {
                        self.expect(Syntax::Special(','))?;
                    }
                }
                self.expect(Syntax::Bracket(']'))?;
                Ok(Expr::List(list))
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
            _ => Err(self.error("Atom")),
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
            self.next();
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
                Syntax::String | Syntax::TripleString | Syntax::Number | Syntax::Bracket('[') => {
                    block.push(self.expr()?);
                }

                // Expression
                Syntax::Word => {
                    if let Some(word) = self.peek() {
                        match word.literal() {
                            "if" => block.push(self.if_expr()?),
                            "for" => block.push(self.for_expr()?),
                            "op!" => {
                                self.next();
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
                _ => return Err(self.error("Block stmt")),
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
                    self.next(); // skip dedent
                    self.next(); // skip else
                    let mut test = Expr::Bool(true);
                    if let Some(word) = self.peek() {
                        if word.literal() == "if" {
                            self.next();
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

        tag.contents = match tag.tag.as_ref() {
            "style" | "script" if tag.attrs.is_empty() => vec![self.raw()?],
            _ => self.block()?,
        };

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

    /// Parse text between INDENT and DEDENT as a single string.
    /// Mostly for <style> and <script>.
    ///
    /// ~ Eventually we'll want to dip into parsing <style> and adding
    /// features there too... ~
    fn raw(&mut self) -> Result<Expr> {
        let mut indents = 0;
        let tok = self.expect(Syntax::Indent)?;
        let start = tok.pos;
        while !self.peek_eof() {
            match self.peek_kind() {
                Syntax::Indent => {
                    self.next();
                    indents += 1;
                }
                Syntax::Dedent => {
                    if indents > 0 {
                        self.next();
                        indents -= 1;
                    } else {
                        break;
                    }
                }
                _ => {
                    self.next();
                }
            }
        }
        let last = self.current();
        let end = last.pos + last.len;
        Ok(Expr::String(self.tokens.source()[start..end].into()))
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
    map
}

// Is the first expression a `<head>`? Used for auto-inserting
// `<html>` and the doctype.
#[inline]
fn first_is_head(block: &[Expr]) -> bool {
    if !block.is_empty() {
        if let Expr::Tag(tag) = &block[0] {
            return tag.tag == "head";
        }
    }
    false
}
