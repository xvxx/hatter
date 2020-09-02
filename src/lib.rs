mod error;

pub use error::{print_error, Error};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Token {
    None,
}

pub fn scan(_src: &str) -> Result<Vec<Token>> {
    Ok(vec![])
}

pub fn parse(_tokens: Vec<Token>) -> Result<()> {
    Ok(())
}
