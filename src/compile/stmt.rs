use crate::{Symbol, Tag, Value};

/// Stmt is an AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    None,
    Value(Value),
    Word(Symbol),
    List(Vec<Stmt>),
    Map(Vec<(Symbol, Stmt)>),
    Call(Box<Stmt>, Vec<Stmt>), // fn, args
    Return(Box<Stmt>),
    If(Vec<(Stmt, Vec<Stmt>)>),
    For(Option<Symbol>, Symbol, Box<Stmt>, Vec<Stmt>), // key, val, iter, body
    While(Box<Stmt>, Vec<Stmt>),
    Assign(Symbol, Box<Stmt>, bool), // var, val, reassign?
    Tag(Tag),
    Fn(Vec<Symbol>, Vec<Stmt>), // args, body
    Args(Vec<(Symbol, Stmt)>),  // keyword args
}

impl From<Tag> for Stmt {
    fn from(tag: Tag) -> Stmt {
        Stmt::Tag(tag)
    }
}

impl From<String> for Stmt {
    fn from(s: String) -> Stmt {
        Stmt::Value(s.into())
    }
}

impl From<&str> for Stmt {
    fn from(s: &str) -> Stmt {
        Stmt::Value(s.into())
    }
}

impl Stmt {
    /// Is this Stmt an actual statement?
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Is this Stmt::None?
    pub fn is_none(&self) -> bool {
        matches!(self, Stmt::None)
    }

    /// If this is a String or a Word, get a &str of its literal value.
    pub fn to_str(&self) -> &str {
        match self {
            Stmt::Value(Value::String(s)) => s.to_str(),
            Stmt::Word(s) => s.to_str(),
            _ => "",
        }
    }

    /// Create a String representation, for debug purposes.
    pub fn to_string(&self) -> String {
        match self {
            Stmt::None => "Stmt::None".to_string(),
            Stmt::Value(Value::String(s)) => format!(r#""{}""#, s),
            Stmt::Value(v) => format!("{}", v),
            Stmt::Word(s) => s.to_string(),
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
            Stmt::While(test, body) => format!("while({:?}) {:?}", test, body),
            Stmt::Fn(args, body) => format!("fn({:?}) {:?}", args, body),
            Stmt::Call(name, args) => format!(
                "{}({})",
                name.to_string(),
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}
