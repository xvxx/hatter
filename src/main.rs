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
        println!("{:#?}", tokens);
        return Ok(());
    }

    let ast = parse(tokens)
        .map_err(|e| print_error(&path, &source, e))
        .unwrap();
    if command == "parse" {
        println!("{:#?}", ast);
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

fn print_error<P: AsRef<std::path::Path>, S: AsRef<str>>(path: P, source: S, err: hatter::Error) {
    hatter::print_error(path, source, err);
    std::process::exit(1)
}
