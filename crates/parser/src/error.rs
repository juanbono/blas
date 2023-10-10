use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub struct ParseError(pub String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing program: {}", self.0)
    }
}
