use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Parsing error: {0}")]
    ParseError(#[from] parser::ParseError),
}
