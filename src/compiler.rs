use {
    crate::{Env, Expr, Result, Value, AST},
    std::collections::HashMap,
};

#[derive(Debug)]
pub enum Code {
    Noop,
    Push(Value),
    Pop,
    Lookup(String),
    JumpTo(usize),
    JumpBy(usize),
    JumpIfTrue(usize),
    JumpIfFalse(usize),
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
        Bool(..) | Number(..) | String(..) => {
            let mut inst = compile_expr(expr)?;
            inst.push(Code::Print);
            inst
        }
        Word(word) => vec![Code::Lookup(word.to_string()), Code::Print],
        Call(name, args) => {
            let mut inst = vec![];
            for expr in args {
                let mut e = compile_expr(expr)?;
                inst.append(&mut e);
            }
            inst.push(Code::Call(name.to_string(), args.len()));
            inst
        }
        If(conds) => {
            let mut inst = vec![];
            let mut ends = vec![]; // needs jump to END

            for (test, body) in conds {
                let mut test = compile_expr(test)?;
                let mut body = compile_exprs(body)?;
                inst.append(&mut test);
                inst.push(Code::JumpIfFalse(body.len()));
                inst.append(&mut body);

                // save this location to rewrite later
                inst.push(Code::JumpBy(0)); // end
                ends.push(inst.len() - 1);
            }

            // rewrite test jumps now that we know how many
            // instructions are in the `else` clauses
            let end_pos = inst.len() - 1;
            for end in ends {
                inst[end] = Code::JumpBy(end_pos - end);
            }

            inst
        }
        _ => unimplemented!(),
        // Tag(Tag),
        // For(Option<String>, String, Box<Expr>, Vec<Expr>), // key, val, iter, body
    })
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
        _ => unimplemented!(),
    })
}
