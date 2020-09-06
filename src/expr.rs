use crate::Tag;

#[derive(Debug)]
pub enum Expr {
    Tag(Tag),
    If,
    For,
    None,
    Bool(bool),
    Number(f64),
    String(String),
    Word(String),
    Call(String, Vec<Expr>),
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::None => "Expr::None".to_string(),
            Expr::Bool(b) => format!("{}", b),
            Expr::Number(n) => format!("{}", n),
            Expr::String(s) => format!(r#""{}""#, s),
            Expr::Word(s) => s.clone(),
            Expr::Tag(tag) => format!("{:?}", tag),
            Expr::If => format!("{}", "IF: Coming Soon™"),
            Expr::For => format!("{}", "FOR: Coming Soon™"),
            Expr::Call(name, args) => format!(
                "{}({})",
                name,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
