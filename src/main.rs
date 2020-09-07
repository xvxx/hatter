use {
    hatter::{compile, parse, scan, vm, Compiled, TokenStream, AST},
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
        command = "print";
        path = &args[0];
    } else {
        command = &args[0];
        path = &args[1];
    }

    match path.as_ref() {
        "-h" | "-help" | "--help" | "help" => {
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
        print_ast(ast);
        return Ok(());
    }

    if command == "check" {
        unimplemented!();
    }

    let compiled = compile(ast)
        .map_err(|e| print_error(&path, &source, e))
        .unwrap();
    if command == "compile" {
        print_compiled(compiled);
        return Ok(());
    }

    // let out = eval(ast)
    //     .map_err(|e| print_error(&path, &source, e))
    //     .unwrap();
    // println!("{}", out);
    Ok(())
}

fn print_usage() {
    print!(
        r#"Usage: hatter [COMMAND] <file.hat>

Commands:
  scan        Print tokens.
  parse       Print AST.
  check       Check for compile errors only.
  compile     Compile to HASM.
  print       Print HTML. (default)
"#
    );
}

fn print_tokens(mut tokens: TokenStream) {
    while let Some(tok) = tokens.next() {
        println!(
            "({:>03}:{:>03}) {:<15} {}",
            tok.pos,
            tok.len,
            format!("{:?}", tok.kind),
            tok.literal()
        );
    }
}

fn print_ast(ast: AST) {
    for expr in ast.exprs {
        println!("{:?}", expr);
    }
}

fn print_compiled(compiled: Compiled) {
    for code in compiled.codes {
        println!("{:?}", code);
    }
}

fn print_error<P: AsRef<std::path::Path>, S: AsRef<str>>(path: P, source: S, err: hatter::Error) {
    hatter::print_error(path, source, err);
    std::process::exit(1)
}
