#![allow(unused)]
#![allow(dead_code)]
use {
    hatter,
    std::{
        fs,
        io::{self, Write},
        path::Path,
    },
};

#[test]
fn test_examples() -> io::Result<()> {
    if shell("which", &["tidy"])? == "" {
        let banner = 50;
        println!("\n{}", "-".repeat(banner));
        println!("Please install tidy to run tests:\n");
        println!("$ brew install tidy");
        println!("{}\n", "-".repeat(banner));
        return Err(io::Error::new(io::ErrorKind::Other, "tidy not found"));
    }
    test_dir("./examples/")
}

fn test_dir<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    let dir = dir.as_ref();
    for test in fs::read_dir(dir)? {
        let test = test?;
        let path = test.path();
        if path.is_dir() {
            test_dir(path)?;
        } else {
            let source = fs::read_to_string(&path)?;

            // some examples are showing off errors
            if path.ends_with("-error.hat") {
                match hatter::render(&source) {
                    Err(..) => {
                        assert!(true);
                        continue;
                    }
                    Ok(..) => assert!(false, "Expected error in {:?} but got OK", path),
                }
            }

            let test_path = format!("{}", path.clone().into_os_string().into_string().unwrap())
                .replace("./examples/", "./tests/examples/")
                .replace(".hat", ".html");

            let tmp_path = "/tmp/hatter.test";
            let mut file = fs::File::create(tmp_path)?;
            match hatter::render(&source) {
                Ok(code) => write!(file, "{}", code).unwrap(),
                Err(err) => {
                    let msg = err.to_string();
                    hatter::print_error(&path, source, err);
                    return Err(io::Error::new(io::ErrorKind::Other, msg));
                }
            }
            let (expected, actual) = (pretty(&test_path)?, pretty(tmp_path)?);
            if expected != actual {
                println!("!!! FAILED: {}", path.display());
                println!("=== EXPECTED ==========\n{}", expected);
                println!("=== ACTUAL ==========\n{}", actual);
                assert!(false);
            }
        }
    }
    Ok(())
}

/// Pretty print the HTML file at `path`.
fn pretty(path: &str) -> io::Result<String> {
    shell("tidy", &["-i", "-q", "--show-body-only", "yes", path])
}

/// Run a script and return its output.
fn shell(path: &str, args: &[&str]) -> io::Result<String> {
    let output = std::process::Command::new(path).args(args).output()?;
    let out = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };
    match std::str::from_utf8(&out) {
        Ok(s) => Ok(s.trim().to_string()),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )),
    }
}
