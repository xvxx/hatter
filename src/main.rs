use {
    hatter::{parse, scan, Stmt, Token, TokenKind, AST},
    std::{env, io},
};

fn main() -> Result<(), io::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let command;
    let path;
    if args.len() < 2 {
        command = "emit";
        path = &args[0];
    } else {
        command = &args[0];
        path = &args[1];
    }

    match path.as_ref() {
        "-h" | "-help" | "--help" => {
            print_usage();
            return Ok(());
        }
        _ => {}
    }

    let source = std::fs::read_to_string(path)?;

    let tokens = scan(&source)
        .map_err(|e| print_error(&path, &source, e))
        .unwrap();
    if command == "scan" {
        print_tokens(tokens);
        return Ok(());
    }

    let ast = parse(tokens)
        .map_err(|e| print_error(&path, &source, e))
        .unwrap();
    if command == "parse" {
        println!("{:#?}", ast);
        // print_ast(ast);
        return Ok(());
    }

    if command == "check" {
        unimplemented!();
    }

    if command == "emit" {
        unimplemented!();
    }

    unimplemented!();
}

fn print_usage() {
    print!(
        r#"Usage: hatter [COMMAND] <file.hat>

Commands:
  scan        Print tokens.
  parse       Print AST.
  check       Check for compile errors only.
  print       Print HTML. (default)
"#
    );
}

/// Pretty-print tokens.
fn print_tokens(tokens: Vec<Token>) {
    let (bold, gold, clear) = if std::env::var("NO_COLOR").is_ok() {
        ("", "", "")
    } else {
        ("\x1b[1m", "\x1b[1;93m", "\x1b[0m")
    };
    let mut indent = 0;
    for (i, token) in tokens.iter().enumerate() {
        if token.kind == TokenKind::Indent {
            println!("");
        } else if token.kind == TokenKind::Dedent {
            indent -= 1;
        }
        let info = format!(
            "{}{:3}\t{}{}{}{:?}{}",
            gold,
            token.pos,
            clear,
            bold,
            "  ".repeat(indent),
            token.kind,
            clear,
        );
        if token.kind == TokenKind::Indent {
            indent += 1;
        }
        println!("{}{}{}", info, " ".repeat(45 - info.len()), token.literal(),);
        if i < tokens.len() - 1 {
            if token.kind == TokenKind::Dedent {
                println!("");
            } else if token.kind == TokenKind::Semi {
                println!("");
            }
        }
    }
}

// Pretty-print ast.
fn print_ast(m: AST) {
    // println!("== {}", m.name);

    // println!("IMPORT: {:?}", m.imports);

    // for s in m.structs {
    //     if let Stmt::Struct(name, fields) = s {
    //         println!("STRUCT {}: {:?}", name, fields);
    //     }
    // }

    // for c in m.consts {
    //     if let Stmt::Const(name, expr) = c {
    //         println!("CONST {} = {:?}", name, expr);
    //     }
    // }

    // for c in m.vars {
    //     if let Stmt::Var(name, expr) = c {
    //         println!("VAR {} = {:?}", name, expr);
    //     }
    // }

    // for s in m.decs {
    //     if let Stmt::Dec(name, params, ty) = s {
    //         println!("DEC {}({:?}) {}", name, params, ty);
    //     }
    // }

    // for s in m.defs {
    //     if let Stmt::Def(name, params, ty, body) = s {
    //         println!("DEF {}({:?}) {}", name, params, ty);
    //         for ex in body.iter() {
    //             println!(
    //                 "\t{}",
    //                 ex.to_string()
    //                     .chars()
    //                     .take(78)
    //                     .map(|c| c.to_string())
    //                     .collect::<Vec<_>>()
    //                     .join("")
    //             );
    //         }
    //     }
    // }

    // println!("== MAIN()");
    // for ex in m.main.iter() {
    //     println!("{}", ex);
    // }
}

fn print_error<P: AsRef<std::path::Path>, S: AsRef<str>>(path: P, source: S, err: hatter::Error) {
    hatter::print_error(path, source, err);
    std::process::exit(1)
}
