use {
    hatter::{compile, parse, scan, vm, Code, Token, AST},
    std::{env, io},
};

fn main() -> Result<(), io::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        return print_usage();
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
            return print_usage();
        }
        "-v" | "-version" | "--version" | "version" => {
            return print_version();
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

    let ast = parse(&tokens)
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
        print_codes(compiled);
        return Ok(());
    }

    let out = vm::run(compiled)
        .map_err(|e| print_error(&path, &source, e))
        .unwrap();
    println!("{}", out);
    Ok(())
}

fn print_usage() -> Result<(), io::Error> {
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
    Ok(())
}

fn print_tokens(mut tokens: Vec<Token>) {
    while !tokens.is_empty() {
        let tok = tokens.remove(0);
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
        println!("{:#?}", expr);
    }
}

fn print_codes(codes: Vec<Code>) {
    for (i, code) in codes.iter().enumerate() {
        println!("{:>04}{}{:?}", i, " ".repeat(3), code);
    }
}

fn print_error<P: AsRef<std::path::Path>, S: AsRef<str>>(path: P, source: S, err: hatter::Error) {
    hatter::print_error(path, source, err);
    std::process::exit(1)
}

/// --version
fn print_version() -> Result<(), io::Error> {
    println!("hatter v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
