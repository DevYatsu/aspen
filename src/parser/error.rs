use std::{error::Error, fmt};

use crate::lexer::LexingError;

pub type AspenResult<T> = Result<T, AspenError>;

#[derive(Debug)]
pub enum AspenError {
    IoError(std::io::Error),
    Lexing(LexingError),

    ExpectedString(String),
    ExpectedSpace,

    Eof,
}

impl Error for AspenError {}

impl fmt::Display for AspenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AspenError::IoError(e) => e.fmt(f),
            AspenError::ExpectedString(s) => write!(f, "{}", s),

            AspenError::Eof => write!(f, "Unexpected end of input!"),
            AspenError::Lexing(e) => e.fmt(f),
            AspenError::ExpectedSpace => write!(f, "Expected a space or newline character"),
        }
    }
}

impl From<LexingError> for AspenError {
    fn from(value: LexingError) -> Self {
        Self::Lexing(value)
    }
}
