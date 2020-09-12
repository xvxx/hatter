mod tag;
pub use tag::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    None,
    Bool(bool),
    Number(usize),
    String(String),
    Word(String),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
    Call(String, Vec<Expr>),
    If(Vec<(Expr, Vec<Expr>)>),
    For(Option<String>, String, Box<Expr>, Vec<Expr>), // key, val, iter, body
    Assign(String, Box<Expr>, bool),                   // var, val, reassign?
    Tag(Tag),
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
            Expr::List(list) => format!(
                "[{}]",
                list.iter()
                    .map(|ex| ex.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Map(map) => format!(
                "{{{}}}",
                map.iter()
                    .map(|(k, v)| format!("{}: {}", k.to_string(), v.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expr::Assign(var, expr, re) => {
                format!("{} {}= {:?}", var, if *re { ":" } else { "" }, expr)
            }
            Expr::If(..) => format!("{}", "IF: Coming Soon™"),
            Expr::For(..) => format!("{}", "FOR: Coming Soon™"),
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
