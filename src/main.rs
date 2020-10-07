use {
    hatter::{parse, render, scan, Stmt, Syntax, Token},
    std::{
        env, fs,
        io::{self, Write},
        path,
    },
};

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        #[cfg(feature = "repl")]
        return hatter::repl::run();
        #[cfg(not(feature = "repl"))]
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

    #[cfg(feature = "repl")]
    if command == "repl" || path == "repl" {
        return hatter::repl::run();
    }

    if !matches!(command, "scan" | "parse" | "check" | "compile" | "print") {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("unknown command: {}", command),
        ));
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

    if !path::Path::new(path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("file not found: {}", path),
        ));
    }

    let source = fs::read_to_string(path)?;

    if command == "scan" {
        let tokens = scan(&source)
            .map_err(|e| print_error(&path, &source, e))
            .unwrap();
        print_tokens(tokens);
        return Ok(());
    }

    if command == "parse" {
        let ast = scan(&source)
            .and_then(|tokens| parse(&tokens))
            .map_err(|e| print_error(&path, &source, e))
            .unwrap();
        print_ast(&ast);
        return Ok(());
    }

    write!(
        io::stdout(),
        "{}",
        render(&source)
            .map_err(|e| print_error(&path, &source, e))
            .unwrap()
    )
}

fn print_usage() -> io::Result<()> {
    #[cfg(not(feature = "repl"))]
    let repl = "";
    #[cfg(feature = "repl")]
    let repl = "  repl        Start REPL. (default without args)\n";
    print!(
        r#"Usage: hatter [COMMAND] <file.hat>

Commands:
  scan        Print tokens.
  parse       Print AST.
  print       Print HTML. (default)
{}"#,
        repl
    );
    Ok(())
}

fn print_tokens(mut tokens: Vec<Token>) {
    let mut indent = 0;
    while !tokens.is_empty() {
        let tok = tokens.remove(0);
        match tok.kind {
            Syntax::Indent => indent += 1,
            Syntax::Dedent => indent -= 1,
            _ => {}
        }
        println!(
            "{}({:>03}:{:>03}) {:<15} {}",
            "  ".repeat(indent),
            tok.pos,
            tok.len,
            format!("{:?}", tok.kind),
            tok.literal()
        );
    }
}

fn print_ast(ast: &[Stmt]) {
    for expr in ast {
        println!("{:#?}", expr);
    }
}

fn print_error<P: AsRef<std::path::Path>, S: AsRef<str>>(path: P, source: S, err: hatter::Error) {
    hatter::print_error(path, source, err);
    std::process::exit(1)
}

/// --version
fn print_version() -> io::Result<()> {
    println!("hatter v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
