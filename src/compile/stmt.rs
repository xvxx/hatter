use crate::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    None,
    Bool(bool),
    Number(isize),
    String(String),
    Word(String),
    List(Vec<Stmt>),
    Map(Vec<(String, Stmt)>),
    Call(String, Vec<Stmt>),
    Return(Box<Stmt>),
    If(Vec<(Stmt, Vec<Stmt>)>),
    For(Option<String>, String, Box<Stmt>, Vec<Stmt>), // key, val, iter, body
    Assign(String, Box<Stmt>, bool),                   // var, val, reassign?
    Tag(Tag),
    Fn(Vec<String>, Vec<Stmt>), // args, body
    Args(Vec<(String, Stmt)>),  // keyword args
}

impl From<Tag> for Stmt {
    fn from(tag: Tag) -> Stmt {
        Stmt::Tag(tag)
    }
}

impl From<String> for Stmt {
    fn from(s: String) -> Stmt {
        Stmt::String(s)
    }
}

impl From<&str> for Stmt {
    fn from(s: &str) -> Stmt {
        Stmt::String(s.to_string())
    }
}

impl Stmt {
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Stmt::None)
    }

    pub fn to_string(&self) -> String {
        match self {
            Stmt::None => "Stmt::None".to_string(),
            Stmt::Bool(b) => format!("{}", b),
            Stmt::Number(n) => format!("{}", n),
            Stmt::String(s) => format!(r#""{}""#, s),
            Stmt::Word(s) => s.clone(),
            Stmt::Tag(tag) => format!("{:?}", tag),
            Stmt::Return(ex) => format!("return {:?}", ex),
            Stmt::Args(args) => args
                .iter()
                .map(|(k, v)| format!("{}: {:?}", k, v))
                .collect::<Vec<_>>()
                .join(", "),
            Stmt::List(list) => format!(
                "[{}]",
                list.iter()
                    .map(|ex| ex.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Stmt::Map(map) => format!(
                "{{{}}}",
                map.iter()
                    .map(|(k, v)| format!("{}: {}", k.to_string(), v.to_string()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Stmt::Assign(var, expr, re) => {
                format!("{} {}= {:?}", var, if *re { ":" } else { "" }, expr)
            }
            Stmt::If(..) => format!("{}", "IF: Coming Soon™"),
            Stmt::For(..) => format!("{}", "FOR: Coming Soon™"),
            Stmt::Fn(args, body) => format!("fn({:?}) {:?}", args, body),
            Stmt::Call(name, args) => format!(
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
