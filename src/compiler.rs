use {
    crate::{Expr, Result, Tag, Value},
    std::collections::HashMap,
};

#[derive(Debug, Clone)]
pub enum Code {
    /// No-op. Just here to help debugging.
    Debug(String),
    /// No-op. Used by compiler to rewrite Jump instructions.
    Label(String),

    /// Print `Value`.
    Print(Value),
    /// Print the value of the variable named `String`.
    PrintVar(String),
    /// Pop the top of the stack and print it.
    PrintPop,

    /// Push `Value` onto stack.
    Push(Value),
    /// Pop the top of the stack.
    Pop,

    /// Create a list of the `usize` top elements on the stack.
    List(usize),
    /// Create a map of the `usize * 2` top elements on the stack,
    /// where the even elements are keys and odd are values.
    Map(usize),

    /// Create a tag with `usize` number of attributes, which has the
    /// same structure as Map: KEY then VALUE.
    /// This pops `usize * 2` off the stack, as well as two more items
    /// for the tag name and self-closing? status.
    ///
    /// This should be the stack when calling:
    ///   09 PUSH "A"              ; tag name
    ///   08 PUSH false            ; self-closing?
    ///   07 PUSH "ID"             ; attr name
    ///   06 PUSH "main-link"      ; attr value
    ///   05 PUSH "CLASS"          ; "class" attr is additive
    ///   04 PUSH "blue"           ; ...
    ///   03 PUSH "HREF"
    ///   02 PUSH "giggle.com"
    ///   01 PUSH "DATA-ITEM-ID"
    ///   00 PUSH 44
    /// Which produces:
    ///   <a id="main-link" class="blue" href="giggle.com" data-item-id="44">
    Tag(usize),
    /// Close the most recently opened tag.
    CloseTag,

    /// Push the value of the variable named `String` onto the stack.
    Lookup(String),
    /// Pops the top of the stack and puts it in the variable named `String`.
    Set(String),
    /// Pops the top of the stack and puts it in the variable named
    /// `String` if the variable already exists.
    SetIfSet(String),

    /// Calls the function named `String` with `usize` arguments,
    /// which need to be popped off the stack.
    Call(String, usize),
    /// End a function call.
    Return,
    /// End the program.
    Exit,

    /// Jumps to a Label. Consumed by the compiler.
    JumpTo(String),
    /// Jumps to a Label if the top of the stack is true. Pops it.
    JumpToIfTrue(String),
    /// Jumps to a Label if the top of the stack is false. Pops it.
    JumpToIfFalse(String),
    /// Jumps by `isize`, relative to the current location.
    JumpBy(String, isize),
    /// Pops the top of the stack and, if it's true, jumps by `isize.`
    JumpByIfTrue(String, isize),
    /// Pops the top of the stack and, if it's false, jumps by `isize.`
    JumpByIfFalse(String, isize),

    /// No-op. Should be handled by the compiler.
    Break,
    /// No-op. Should be handled by the compiler.
    Continue,

    /// Used to start a loop. TODO: remove
    InitLoop,
    /// Run at the top of each loop iteration. TODO: remove
    Loop(Option<String>, String),
    /// Used before `EndLoop`, to see if we should jump. TODO: remove
    TestShouldLoop,
    /// Used to end a loop. TODO: remove
    EndLoop,
}

pub fn compile(ast: &[Expr]) -> Result<Vec<Code>> {
    let mut compiler = Compiler::new();
    compiler.compile(ast.into())
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

    fn compile(&mut self, ast: &[Expr]) -> Result<Vec<Code>> {
        let mut codes = vec![];
        for expr in ast {
            let mut ex = self.compile_stmt(expr)?;
            codes.append(&mut ex);
        }
        codes.push(Code::Exit); // fin
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
            Assign(var, expr, reassign) => {
                let mut inst = self.compile_expr(expr)?;
                if *reassign {
                    inst.push(Code::SetIfSet(var.clone()));
                } else {
                    inst.push(Code::Set(var.clone()));
                }
                inst
            }
            Word(word) => match word.as_ref() {
                "break" => vec![Code::Break],
                "continue" => vec![Code::Continue],
                _ => vec![Code::PrintVar(word.clone())],
            },
            Return(ex) => {
                if let Expr::None = **ex {
                    vec![Code::Return]
                } else {
                    let mut inst = self.compile_expr(ex)?;
                    inst.push(Code::Return);
                    inst
                }
            }
            Call(..) | List(..) | Map(..) | Fn(..) | Tag(..) => {
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

    // Stack should be:
    // - TAG NAME
    // - BOOL: CLOSED?
    // - ATTR NAME
    // - ATTR VALUE
    // - TAG-OP (# of attrs)
    fn compile_tag(&mut self, tag: &Tag) -> Result<Vec<Code>> {
        let mut inst = self.compile_expr(&tag.tag)?; // TAG NAME
        inst.push(Code::Push(tag.closed.into())); // CLOSED?

        // need to know if it's a <form>
        let is_form = if let Expr::String(s) = &*tag.tag {
            s == "form"
        } else {
            false
        };

        // ID
        let mut id = self.compile_expr(&tag.id)?;
        if !id.is_empty() {
            inst.push(Code::Push("id".into()));
            inst.append(&mut id);
        }

        // CLASSES
        for class in &tag.classes {
            inst.push(Code::Push("class".into()));
            inst.append(&mut self.compile_expr(class)?);
        }

        // ATTRS
        let mut attr_len = 0;
        for (name, val) in &tag.attrs {
            attr_len += 1;
            if is_form {
                if let Expr::String(s) = name {
                    if s == "GET" || s == "POST" {
                        attr_len += 1;
                        inst.push(Code::Push("method".into()));
                        inst.push(Code::Push(s.into()));
                        inst.push(Code::Push("target".into()));
                        inst.append(&mut self.compile_expr(val)?);
                        continue;
                    }
                }
            }

            inst.append(&mut self.compile_expr(&name)?);
            inst.append(&mut self.compile_expr(&val)?);
        }

        inst.push(Code::Tag(
            attr_len + tag.classes.len() + if let Expr::None = &*tag.id { 0 } else { 1 },
        ));

        if !tag.closed {
            if !tag.body.is_empty() {
                inst.append(&mut self.compile_stmts(&tag.body)?);
            }
            inst.push(Code::CloseTag);
        }

        Ok(inst)
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<Vec<Code>> {
        use Expr::*;

        Ok(match expr {
            None => vec![],
            Bool(b) => vec![Code::Push(b.into())],
            Number(n) => vec![Code::Push(n.into())],
            String(s) => vec![Code::Push(s.into())],
            Word(word) => vec![Code::Lookup(word.to_string())],
            Tag(tag) => self.compile_tag(tag)?,
            List(list) => {
                let mut inst = vec![];
                for expr in list {
                    let mut code = self.compile_expr(expr)?;
                    inst.append(&mut code);
                }
                inst.push(Code::List(list.len()));
                inst
            }
            Map(map) => {
                let mut inst = vec![];
                for (key, val) in map {
                    inst.push(Code::Push(key.into()));
                    let mut expr = self.compile_expr(val)?;
                    inst.append(&mut expr);
                }
                inst.push(Code::Map(map.len()));
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
            Fn(params, body) => vec![Code::Push(Value::Fn {
                params: params.clone(),
                body: self.compile_stmts(body)?,
            })],
            _ => panic!("Not an expression: {:?}", expr),
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
