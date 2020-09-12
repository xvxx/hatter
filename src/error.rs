#![allow(unused_macros)]
use std::{error, fmt, io};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub details: String,
    pub pos: usize,
    pub len: usize,
}

impl Error {
    pub fn new(details: String, pos: usize, len: usize) -> Error {
        Error { details, pos, len }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.details)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            details: format!("{}", error),
            pos: 0,
            len: 0,
        }
    }
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, error.details)
    }
}

/// Create an error with line and col information.
macro_rules! scan_error {
    ($pos:expr, $len:expr, $msg:expr) => {{
        use crate::Error;
        Err(Error::new($msg.into(), $pos, $len))
    }};
    ($pos:expr, $len:expr, $msg:expr, $($args:expr),+) => {
        scan_error!($pos, $len, format!($msg, $($args),*));
    };
}

/// Create an error at a position in the source.
macro_rules! pos_error {
    ($pos:expr, $msg:expr) => {{
        use crate::Error;
        Err(Error::new($msg.into(), $pos, 1))
    }};
    ($msg:expr, $($args:expr),*) => {
        pos_error!(format!($msg, $($args),*));
    };
}

/// Convenient way to create an Err(Error{}).
macro_rules! error {
    ($msg:expr) => {{
        use crate::Error;
        Err(Error::new($msg.into(), 0, 0))
    }};
    ($msg:expr, $($args:expr),*) => {
        error!(format!($msg, $($args),*));
    };
}

/// Pretty-print an error message, complete with colors and the line
/// in question.
pub fn print_error<P: AsRef<std::path::Path>, S: AsRef<str>>(path: P, source: S, err: Error) {
    let path = path.as_ref();
    let source = source.as_ref();
    let (red, blue, _gold, clear) = if std::env::var("NO_COLOR").is_ok() {
        ("", "", "", "")
    } else {
        ("\x1b[91m", "\x1b[1;94m", "\x1b[1;93m", "\x1b[0m")
    };

    let (line, col) = line_and_col(source, err.pos);
    // println!("(line, col) = {:?}", (line, col));
    println!(
        " {}-->{} {}:{}:{}",
        blue,
        clear,
        path.to_str().unwrap(),
        line,
        col
    );
    println!("   {}|{}", blue, clear);

    let lines = source.split("\n");
    let pline = |num| {
        if let Some(line) = lines.clone().nth(num) {
            println!("{}   |{} {}", blue, clear, line)
        }
    };

    if line > 2 {
        pline(line - 3);
    }
    if line > 1 {
        pline(line - 2);
    }
    println!(
        "{}{: <3}|{} {}",
        blue,
        line,
        clear,
        lines.clone().nth(line - 1).unwrap()
    );
    println!(
        "   {}|{} {}{} {}{}",
        blue,
        red,
        " ".repeat(if col > 0 { col - 1 } else { 0 }),
        "^".repeat(std::cmp::min(err.len, 20)),
        err.details,
        clear
    );
    pline(line);
    pline(line + 1);
}

/// Calculate line # and col position for a position in a source file.
fn line_and_col(source: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 0;
    for (i, c) in source.chars().enumerate() {
        if c == '\n' {
            if i == pos {
                return (line, col + 1);
            }
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        if i >= pos {
            break;
        }
    }
    (line, col)
}
