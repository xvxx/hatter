/// The different types of syntax that our `Lexer` recognizes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Syntax {
    None,
    Indent,
    Dedent,
    Bool(bool),   // true false
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
    LCaret,       // <
    RCaret,       // >
    Slash,        // / (only in <tags>)
    Equal,        // = (only in <tags>)

    // Keywords
    Def,
    Do,
    Return,
    If,
    Then,
    Else,
    For,
    While,
    In,
    Fn,
}

impl Syntax {
    /// Does this syntax start an expression?
    #[rustfmt::skip]
    pub fn starts_expr(&self) -> bool {
        matches!(self,
            Syntax::String(..) | Syntax::Bool(..) | Syntax::Number | Syntax::Word | Syntax::Fn |
            Syntax::Op | Syntax::LCaret | Syntax::LParen | Syntax::LStaple | Syntax::LCurly
        )
    }

    /// Is this a word or a keyword? Used for implicit body text in tags.
    pub fn is_word_or_keyword(&self) -> bool {
        use Syntax::*;
        match self {
            Word | Def | Do | Return | If | Then | Else | For | While | In | Fn => true,
            None | Indent | Dedent | Bool(..) | Number | String(..) | JS | Op | Semi | Colon
            | Comma | LParen | RParen | LCurly | RCurly | LStaple | RStaple | LCaret | RCaret
            | Slash | Equal => false,
        }
    }
}

/// Helper method we add to `char`.
pub trait SyntaxTrait {
    fn is_word_char(&self) -> bool;
    fn is_tag_opener(&self) -> bool;
    fn is_op(&self) -> bool;
    fn is_bracket(&self) -> bool;
}

impl SyntaxTrait for char {
    /// Is this char valid in a `Syntax::Word`?
    fn is_word_char(&self) -> bool {
        self.is_alphanumeric() || matches!(*self, '-' | '_' | '\'')
    }

    /// Is this valid in the opening section of a <tag>?
    fn is_tag_opener(&self) -> bool {
        self.is_alphabetic() || matches!(self, '#' | '.' | ':' | '@' | '/')
    }

    /// Operator?
    fn is_op(&self) -> bool {
        !self.is_whitespace()
            && !self.is_alphanumeric()
            && !self.is_bracket()
            && !matches!(*self, '#' | '"' | '`' | '\'')
    }

    /// ( ) [ ] { }
    fn is_bracket(&self) -> bool {
        matches!(self, '(' | ')' | '[' | ']' | '{' | '}')
    }
}
