use crate::{Error, ErrorKind, Result, Symbol, Syntax};

#[derive(Debug, Clone, Copy)]
pub struct Token<'s> {
    pub pos: usize,
    pub len: usize,
    pub kind: Syntax,
    lit: &'s str,
}

impl<'s> Token<'s> {
    /// Create a Token.
    pub fn new(kind: Syntax, pos: usize, len: usize, lit: &'s str) -> Token<'s> {
        Token {
            kind,
            pos,
            len,
            lit,
        }
    }

    /// Will it produce a `literal()`?
    pub fn is_literal(&self) -> bool {
        use Syntax::*;
        // We want this to fail if new variants are added.
        match self.kind {
            None | Bool(..) | Number | String(..) | Word | JS | Op | Colon | Comma | LParen
            | RParen | LCurly | RCurly | LStaple | RStaple | LCaret | RCaret | Slash | Equal
            | Def | Do | Return | If | Then | Else | For | While | In | Fn => true,
            Indent | Dedent | Semi => false,
        }
    }

    /// Get the literal value in source code, if it has one.
    pub fn literal(&self) -> &str {
        if self.is_literal() {
            self.lit
        } else {
            ""
        }
    }

    /// Also...
    pub fn to_str(&self) -> &str {
        self.literal()
    }

    /// Intern string.
    pub fn to_sym(&self) -> Symbol {
        Symbol::from(self.to_str())
    }

    /// Create a string of the literal value.
    pub fn to_string(&self) -> String {
        self.literal().to_string()
    }

    /// Convert into native number or error. No weak typing.
    pub fn to_f64(&self) -> Result<f64> {
        if self.literal().len() > 2 {
            match &self.literal()[..2] {
                "0b" => return Ok(self.from_bin()? as f64),
                "0o" => return Ok(self.from_oct()? as f64),
                "0x" => return Ok(self.from_hex()? as f64),
                _ => {}
            }
        }
        self.literal()
            .replace('_', "")
            .parse::<f64>()
            .map_err(|e| Error::new(ErrorKind::ParseError, e.to_string(), self.pos, 1))
    }

    /// 0b10101 to isize
    fn from_bin(&self) -> Result<isize> {
        if self.literal().starts_with("0b") {
            Ok(isize::from_str_radix(&self.literal()[2..], 2)?)
        } else {
            error!("can't parse binary number: {:?}", self)
        }
    }

    /// 0o755 to isize
    fn from_oct(&self) -> Result<isize> {
        if self.literal().starts_with("0o") {
            Ok(isize::from_str_radix(&self.literal()[2..], 8)?)
        } else {
            error!("can't parse binary number: {:?}", self)
        }
    }

    /// 0xdeadbeef to isize
    fn from_hex(&self) -> Result<isize> {
        if self.literal().starts_with("0x") {
            Ok(isize::from_str_radix(&self.literal()[2..], 16)?)
        } else {
            error!("can't parse binary number: {:?}", self)
        }
    }
}
