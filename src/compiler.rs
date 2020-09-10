use {
    crate::{Env, Expr, Result, Tag, Value, AST},
    std::collections::HashMap,
};

#[derive(Debug)]
pub enum Code {
    Debug(String),
    Label(String),
    Incr(String),
    Decr(String),
    Push(Value),
    Print(Value),
    PrintVar(String),
    PrintPop,
    Pop,
    List(usize),
    Lookup(String),
    Set(String),
    JumpTo(String),
    JumpToIfTrue(String),
    JumpToIfFalse(String),
    Jump(usize),
    JumpBy(String, isize),
    JumpByIfTrue(String, isize),
    JumpByIfFalse(String, isize),
    InitLoop,
    EndLoop,
    Loop(Option<String>, String),
    TestShouldLoop,
    Break,
    Continue,
    Call(String, usize),
    Exit,
    Return,
}

pub fn compile(ast: AST) -> Result<Vec<Code>> {
    let mut compiler = Compiler::new();
    compiler.compile(ast)
}

pub struct Compiler {
    label: usize,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler { label: 0 }
    }

    fn label(&mut self, s: &str) -> String {
        let label = self.label;
        self.label += 1;
        format!("{}_{}", s, label)
    }

    fn compile(&mut self, ast: AST) -> Result<Vec<Code>> {
        let mut codes = vec![];
        for expr in &ast.exprs {
            let mut ex = self.compile_stmt(expr)?;
            codes.append(&mut ex);
        }
        self.reconcile_labels(codes)
    }

    fn compile_stmts(&mut self, exprs: &[Expr]) -> Result<Vec<Code>> {
        let mut out = vec![];
        for expr in exprs {
            let mut e = self.compile_stmt(expr)?;
            out.append(&mut e);
        }
        Ok(out)
    }

    fn compile_stmt(&mut self, expr: &Expr) -> Result<Vec<Code>> {
        use Expr::*;

        Ok(match expr {
            None => vec![],
            Bool(b) => vec![Code::Print(b.into())],
            Number(n) => vec![Code::Print(n.into())],
            String(s) => vec![Code::Print(s.into())],
            Tag(tag) => self.compile_tag(tag)?,
            Assign(var, expr, _) => {
                let mut inst = self.compile_expr(expr)?;
                inst.push(Code::Set(var.clone()));
                inst
            }
            Word(word) => match word.as_ref() {
                "break" => vec![Code::Break],
                "continue" => vec![Code::Continue],
                _ => vec![Code::PrintVar(word.clone())],
            },
            Call(..) | List(..) => {
                let mut inst = self.compile_expr(expr)?;
                inst.push(Code::PrintPop);
                inst
            }
            If(conds) => {
                let mut inst = vec![];
                let end_label = self.label("end_if");
                let mut next_label = self.label("if_cond");

                for (test, body) in conds {
                    let mut test = self.compile_expr(test)?;
                    let mut body = self.compile_stmts(body)?;
                    inst.push(Code::Label(next_label.clone()));
                    next_label = self.label("if_cond");
                    inst.append(&mut test);
                    inst.push(Code::JumpToIfFalse(next_label.clone()));
                    inst.append(&mut body);
                    inst.push(Code::JumpTo(end_label.clone()));
                }
                inst.push(Code::Label(next_label));
                inst.push(Code::Label(end_label));

                inst
            }
            // key, val, iter, body
            // Option<String>, String, Box<Expr>, Vec<Expr>
            For(key, val, iter, body) => {
                let mut inst = vec![];
                let start_label = self.label("loop_start");
                let continue_label = self.label("loop_continue");
                let end_label = self.label("loop_end");
                let mut expr = self.compile_expr(iter)?;
                let mut body = self
                    .compile_stmts(body)?
                    .into_iter()
                    .map(|code| match code {
                        Code::Break => Code::JumpTo(end_label.clone()),
                        Code::Continue => Code::JumpTo(continue_label.clone()),
                        _ => code,
                    })
                    .collect::<Vec<_>>();

                inst.append(&mut expr); // push list
                inst.push(Code::InitLoop);
                inst.push(Code::Label(start_label.clone()));
                inst.push(Code::Loop(key.clone(), val.clone())); // setup loop over list
                inst.append(&mut body); // run code
                inst.push(Code::Label(continue_label));
                inst.push(Code::TestShouldLoop);
                inst.push(Code::JumpToIfTrue(start_label));
                inst.push(Code::Label(end_label));
                inst.push(Code::EndLoop);
                inst
            }
        })
    }

    fn compile_tag(&mut self, tag: &Tag) -> Result<Vec<Code>> {
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
            return Ok(vec![Code::Print(out.into())]);
        } else {
            out.push('>');
        }

        let mut inst = vec![Code::Print(out.into())];

        if !tag.contents.is_empty() {
            let mut body = self.compile_stmts(&tag.contents)?;
            inst.append(&mut body);
        }

        inst.push(Code::Print(format!("</{}>", tag.tag).into()));

        Ok(inst)
    }

    fn compile_exprs(&mut self, exprs: &[Expr]) -> Result<Vec<Code>> {
        let mut out = vec![];
        for expr in exprs {
            let mut e = self.compile_expr(expr)?;
            out.append(&mut e);
        }
        Ok(out)
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<Vec<Code>> {
        use Expr::*;

        Ok(match expr {
            None => vec![],
            Bool(b) => vec![Code::Push(b.into())],
            Number(n) => vec![Code::Push(n.into())],
            String(s) => vec![Code::Push(s.into())],
            Word(word) => vec![Code::Lookup(word.to_string())],
            List(list) => {
                let mut inst = vec![];
                for expr in list {
                    let mut code = self.compile_expr(expr)?;
                    inst.append(&mut code);
                }
                inst.push(Code::List(list.len()));
                inst
            }
            Call(name, args) => {
                let mut inst = vec![];
                for expr in args {
                    let mut e = self.compile_expr(expr)?;
                    inst.append(&mut e);
                }
                inst.push(Code::Call(name.to_string(), args.len()));
                inst
            }
            _ => panic!("don't know how to compile {:?}", expr),
        })
    }

    fn reconcile_labels(&mut self, mut codes: Vec<Code>) -> Result<Vec<Code>> {
        let mut labels = HashMap::new();
        let mut wants_label = vec![];
        for (i, code) in codes.iter().enumerate() {
            match code {
                Code::Label(lbl) => {
                    labels.insert(lbl.clone(), i);
                }
                Code::JumpTo(..) | Code::JumpToIfTrue(..) | Code::JumpToIfFalse(..) => {
                    wants_label.push(i);
                }
                _ => {}
            }
        }
        for idx in wants_label {
            let new = match &codes[idx] {
                Code::JumpTo(lbl) => {
                    let pos = *labels.get(lbl).unwrap_or(&0) as isize;
                    Code::JumpBy(lbl.clone(), pos - idx as isize)
                }
                Code::JumpToIfTrue(lbl) => {
                    let pos = *labels.get(lbl).unwrap_or(&0) as isize;
                    Code::JumpByIfTrue(lbl.clone(), pos - idx as isize)
                }
                Code::JumpToIfFalse(lbl) => {
                    let pos = *labels.get(lbl).unwrap_or(&0) as isize;
                    Code::JumpByIfFalse(lbl.clone(), pos - idx as isize)
                }
                _ => unimplemented!(),
            };
            codes[idx] = new;
        }
        Ok(codes)
    }
}
