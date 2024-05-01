use std::{error::Error, fmt};

use crate::{evaluate::error::EvaluateError, lexer::LexingError};

use super::AspenParser;

pub type AspenResult<T> = Result<T, AspenError>;

#[derive(Debug)]
pub enum AspenError {
    IoError(std::io::Error),
    Lexing {
        error: LexingError,
        start: usize,
        end: usize,
        length: usize,
    },
    Evaluate {
        error: EvaluateError,
        start: usize,
        end: usize,
        length: usize,
        note: String,
    },

    ExpectedSpace {
        start: usize,
        end: usize,
        length: usize,
    },
    ExpectedNewline {
        start: usize,
        end: usize,
        length: usize,
    },
    Expected {
        error: String,
        start: usize,
        end: usize,
        length: usize,
    },
    Unknown {
        error: String,
        start: usize,
        end: usize,
        length: usize,
    },

    Eof,
}

impl Error for AspenError {}

impl fmt::Display for AspenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AspenError::IoError(e) => e.fmt(f),
            AspenError::Lexing { error, .. } => write!(f, "{}", error),
            AspenError::Evaluate { error, .. } => write!(f, "{}", error),

            AspenError::Eof => write!(f, "Unexpected end of input!"),

            AspenError::Expected { error, .. } => write!(f, "Expected {}", error),
            AspenError::ExpectedSpace { .. } => write!(f, "Expected a space character"),
            AspenError::ExpectedNewline { .. } => write!(f, "Expected a newline character"),
            AspenError::Unknown { error, .. } => write!(f, "Unknown {}", error),
        }
    }
}

impl<'a> AspenError {
    pub fn from_lexing_error(parser: &mut AspenParser<'a>, error: LexingError) -> Self {
        let span = parser.lexer.span();
        Self::Lexing {
            error,
            start: span.start,
            end: span.end,
            length: parser.lexer.slice().len(),
        }
    }
    pub fn from_evaluate_error(
        error: EvaluateError,
        note: String,
        start: usize,
        end: usize,
        length: usize,
    ) -> Self {
        Self::Evaluate {
            error,
            start,
            end,
            length,
            note,
        }
    }

    pub fn expected(parser: &mut AspenParser<'a>, error: String) -> Self {
        let span = parser.lexer.span();
        Self::Expected {
            error,
            start: span.start,
            end: span.end,
            length: parser.lexer.slice().len(),
        }
    }

    pub fn expected_space(parser: &mut AspenParser<'a>) -> Self {
        let span = parser.lexer.span();
        Self::ExpectedSpace {
            start: span.start,
            end: span.end,
            length: parser.lexer.slice().len(),
        }
    }

    pub fn expected_newline(parser: &mut AspenParser<'a>) -> Self {
        let span = parser.lexer.span();
        Self::ExpectedNewline {
            start: span.start,
            end: span.end,
            length: parser.lexer.slice().len(),
        }
    }

    pub fn unknown(parser: &mut AspenParser<'a>, error: String) -> Self {
        let span = parser.lexer.span();
        Self::Unknown {
            error,
            start: span.start,
            end: span.end,
            length: parser.lexer.slice().len(),
        }
    }
    pub fn eof(parser: &mut AspenParser<'a>) -> Self {
        Self::Eof
    }
}

impl From<std::io::Error> for AspenError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
