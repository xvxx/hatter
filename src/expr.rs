#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    String(String),
    Word(String),
    Number(f64),
    Call(String, Vec<Expr>),
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::String(s) | Expr::Word(s) => s.clone(),
            Expr::Bool(b) => format!("{}", b),
            Expr::Number(n) => format!("{}", n),
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
