use crate::{Error, Result, Syntax};

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
            None | Number | String(..) | Word | JS | Op | Colon | Comma | LParen | RParen
            | LCurly | RCurly | LStaple | RStaple | LCaret | RCaret => true,
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

    /// Create a string of the literal value.
    pub fn to_string(&self) -> String {
        self.literal().to_string()
    }

    /// Convert into native number or error. No weak typing.
    pub fn to_isize(&self) -> Result<isize> {
        if self.literal().len() > 2 {
            match &self.literal()[..2] {
                "0b" => return self.from_bin(),
                "0o" => return self.from_oct(),
                "0x" => return self.from_hex(),
                _ => {}
            }
        }
        self.literal()
            .replace('_', "")
            .parse::<isize>()
            .map_err(|e| Error::new(e.to_string(), self.pos, 1))
    }

    /// Convert into native number or error. No weak typing.
    pub fn to_f64(&self) -> Result<f64> {
        self.literal()
            .parse::<f64>()
            .map_err(|e| Error::new(e.to_string(), self.pos, 1))
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
