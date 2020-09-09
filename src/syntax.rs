#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Syntax {
    None,
    Indent,
    Dedent,
    Bracket(char), // < > ( ) [ ] { }
    Special(char), // ; # . , @ : = /
    Bool,          // true false
    Number,        // 123 3.14
    String,        // "string" 'or string' `or string`
    TripleString,  // """string"""
    Word,          // ident, symbol, etc
    JS,            // (JavaScript)
}
