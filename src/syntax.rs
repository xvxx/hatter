#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Syntax {
    None,
    Indent,
    Dedent,
    Number,       // 123 3.14
    String(bool), // "string", bool is `interpolated?` or not
    Word,         // ident, symbol, etc
    JS,           // (JavaScript)
    Op,           // + - := etc
    Semi,         // ;
    Colon,        // :
    Comma,        // ,
    LParen,       // (
    RParen,       // )
    LCurly,       // {
    RCurly,       // }
    LStaple,      // [
    RStaple,      // ]
    LessThan,     // <
    GreaterThan,  // >
}

pub trait WordChar {
    fn is_word_char(&self) -> bool;
}

impl WordChar for char {
    /// Is this char valid in a `Syntax::Word`?
    fn is_word_char(&self) -> bool {
        self.is_alphanumeric() || *self == '-' || *self == '_'
    }
}
