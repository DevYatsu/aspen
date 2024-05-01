use super::types::AspenType;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum EvaluateError {
    UndefinedIdentifier(String),
    IdentifierAlreadyUsed(String),

    InvalidType {
        expected: AspenType,
        found: AspenType,
    },

    IdentifierIsNotValidFn(String),
    OnlyFuncsCanBeCalled(String),
    NotEnoughArgs {
        expected_num: usize,
        found: usize,
    },
    TooMuchArgs {
        expected_num: usize,
        found: usize,
    },
}

impl<'a> Error for EvaluateError {}

impl fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvaluateError::IdentifierAlreadyUsed(name) => {
                write!(f, "Identifier already in use: '{}'", name)
            }
            EvaluateError::UndefinedIdentifier(name) => {
                write!(f, "Unknown variable or function: '{}'", name)
            }
            EvaluateError::InvalidType { expected, found } => {
                write!(
                    f,
                    "Invalid type: expected type '{}' found type '{}'",
                    expected, found
                )
            }
            EvaluateError::IdentifierIsNotValidFn(name) => {
                write!(f, "Func '{}' cannot be called as it does not exist!", name)
            }
            EvaluateError::OnlyFuncsCanBeCalled(expr) => {
                write!(
                    f,
                    "<expr> '{}' is not a valid function, it cannot be called!",
                    expr
                )
            }
            EvaluateError::NotEnoughArgs {
                expected_num,
                found,
            } => {
                write!(
                    f,
                    "Invalid number of arguments when invoking function. Expected at least {}, found {}",
                    expected_num, found
                )
            }
            EvaluateError::TooMuchArgs {
                expected_num,
                found,
            } => {
                write!(
                    f,
                    "Invalid number of arguments when invoking function. Expected at most {}, found {}",
                    expected_num, found
                )
            }
        }
    }
}
