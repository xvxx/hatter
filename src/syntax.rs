#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Syntax {
    None,
    Indent,
    Dedent,
    Bracket(char), // < > ( ) [ ] { }
    Special(char), // ; # . , @ : = /
    Bool,          // true false
    Number,        // 123 3.14
    String(bool),  // "string", bool is `interpolated?` or not
    Word,          // ident, symbol, etc
    JS,            // (JavaScript)
}

pub trait Reserved {
    fn is_reserved(&self) -> bool;
}

impl Reserved for char {
    #[rustfmt::skip]
    fn is_reserved(&self) -> bool {
        matches!(self, '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' |
            ';' | ',' | '.' | '#' | '@' | ':' | '=' | '/' )
    }
}
