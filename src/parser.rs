use crate::{Result, TokenStream, AST};

pub struct Parser {
    ast: AST,
    tokens: TokenStream,
    pos: usize,
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
        }
    }

    /// Parse `TokenStream` into `AST`.
    pub fn parse(&mut self) -> Result<()> {
        Ok(())
    }
}
