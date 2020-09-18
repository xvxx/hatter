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

pub trait SyntaxTrait {
    fn is_word_char(&self) -> bool;
    fn is_tag_opener(&self) -> bool;
}

impl SyntaxTrait for char {
    /// Is this char valid in a `Syntax::Word`?
    fn is_word_char(&self) -> bool {
        self.is_alphanumeric() || *self == '-' || *self == '_'
    }

    /// Is this valid in the opening section of a <tag>?
    fn is_tag_opener(&self) -> bool {
        self.is_alphanumeric() || matches!(self, '#' | '.' | ':' | '@')
    }
}
