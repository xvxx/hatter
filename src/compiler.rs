use {
    crate::{Env, Expr, Result, Tag, Value, AST},
    std::collections::HashMap,
};

#[derive(Debug)]
pub enum Code {
    Noop,
    Incr(String),
    Decr(String),
    Push(Value),
    Pop,
    Lookup(String),
    Set(String),
    JumpTo(usize),
    JumpBy(isize),
    JumpIfTrue(isize),
    JumpIfFalse(isize),
    Loop(Option<String>, String),
    ShouldLoop,
    Call(String, usize),
    Print,
    Exit,
    Return,
}

pub struct Compiled<'p> {
    pub codes: Vec<Code>,
    pub env: Env<'p>,
    pub builtins: HashMap<String, Box<dyn Fn(&mut Env, &[Value]) -> Result<Value>>>,
}

impl<'p> Compiled<'p> {
    fn new() -> Compiled<'p> {
        Compiled {
            codes: vec![],
            env: Env::root(),
            builtins: HashMap::new(),
        }
    }

    fn push_codes(&mut self, mut codes: Vec<Code>) {
        self.codes.append(&mut codes);
    }
}

struct Compiler;

pub fn compile<'p>(ast: AST) -> Result<Compiled<'p>> {
    let mut code = Compiled::new();
    for expr in &ast.exprs {
        code.push_codes(compile_stmt(expr)?);
    }
    Ok(code)
}

fn compile_stmts(exprs: &[Expr]) -> Result<Vec<Code>> {
    let mut out = vec![];
    for expr in exprs {
        let mut e = compile_stmt(expr)?;
        out.append(&mut e);
    }
    Ok(out)
}

fn compile_stmt(expr: &Expr) -> Result<Vec<Code>> {
    use Expr::*;

    Ok(match expr {
        None => vec![],
        Bool(..) | Number(..) | String(..) | Call(..) => {
            let mut inst = compile_expr(expr)?;
            inst.push(Code::Print);
            inst
        }
        Word(word) => vec![Code::Lookup(word.to_string()), Code::Print],
        If(conds) => {
            let mut inst = vec![];
            let mut ends = vec![]; // needs jump to END

            for (test, body) in conds {
                let mut test = compile_expr(test)?;
                let mut body = compile_stmts(body)?;
                inst.append(&mut test);
                inst.push(Code::JumpIfFalse(body.len() as isize));
                inst.append(&mut body);

                // save this location to rewrite later
                inst.push(Code::JumpBy(0)); // end
                ends.push(inst.len() - 1);
            }

            // rewrite test jumps now that we know how many
            // instructions are in the `else` clauses
            let end_pos = inst.len() - 1;
            for end in ends {
                inst[end] = Code::JumpBy((end_pos - end) as isize);
            }

            inst
        }
        // key, val, iter, body
        // Option<String>, String, Box<Expr>, Vec<Expr>
        For(key, val, iter, body) => {
            let mut inst = vec![];
            let mut expr = compile_expr(iter)?;
            let mut body = compile_stmts(body)?;
            inst.append(&mut expr); // push list
            inst.push(Code::Loop(key.clone(), val.clone())); // setup loop over list
            inst.append(&mut body); // run code
            inst.push(Code::ShouldLoop);
            inst.push(Code::JumpIfTrue(-(body.len() as isize)));
            inst
        }
        Tag(tag) => compile_tag(tag)?,
        _ => unimplemented!(),
    })
}

fn compile_tag(tag: &Tag) -> Result<Vec<Code>> {
    let mut out = String::new();
    out.push('<');
    let is_form = tag.tag == "form";
    out.push_str(&tag.tag);

    if !tag.classes.is_empty() {
        out.push_str(" class='");
        let len = tag.classes.len();
        for (i, class) in tag.classes.iter().enumerate() {
            out.push_str(class);
            if i < len - 1 {
                out.push(' ');
            }
        }
        out.push_str("'");
    }

    for (name, val) in &tag.attrs {
        if is_form && (name == "GET" || name == "POST") {
            out.push_str(&format!(" method='{}' action='{}'", name, val));
            continue;
        }
        out.push(' ');
        out.push_str(&name);
        out.push('=');
        out.push('\'');
        out.push_str(&val);
        out.push('\'');
    }

    if tag.tag == "a" && !tag.attrs.contains_key("href") {
        out.push_str(" href='#'");
    }

    if tag.is_closed() {
        out.push('/');
        out.push('>');
        return Ok(vec![Code::Push(out.into()), Code::Print]);
    } else {
        out.push('>');
    }

    let mut inst = vec![Code::Push(out.into()), Code::Print];

    if !tag.contents.is_empty() {
        let mut body = compile_stmts(&tag.contents)?;
        inst.append(&mut body);
    }

    inst.push(Code::Push(format!("</{}>", tag.tag).into()));
    inst.push(Code::Print);

    Ok(inst)
}

fn compile_exprs(exprs: &[Expr]) -> Result<Vec<Code>> {
    let mut out = vec![];
    for expr in exprs {
        let mut e = compile_expr(expr)?;
        out.append(&mut e);
    }
    Ok(out)
}

fn compile_expr(expr: &Expr) -> Result<Vec<Code>> {
    use Expr::*;

    Ok(match expr {
        None => vec![],
        Bool(b) => vec![Code::Push(b.into())],
        Number(n) => vec![Code::Push(n.into())],
        String(s) => vec![Code::Push(s.into())],
        Call(name, args) => {
            let mut inst = vec![];
            for expr in args {
                let mut e = compile_expr(expr)?;
                inst.append(&mut e);
            }
            inst.push(Code::Call(name.to_string(), args.len()));
            inst
        }
        _ => unimplemented!(),
    })
}
