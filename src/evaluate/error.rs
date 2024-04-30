use std::{error::Error, fmt};

use crate::parser::error::AspenError;

#[derive(Debug)]
pub enum EvaluateError {
    UnknownFunc(String),
    FuncAlreadyDefined(String),

    UnknownVar(String),
    VarAlreadyDefined(String),
}

impl Error for EvaluateError {}

impl fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvaluateError::UnknownFunc(name) => write!(f, "Unknown function: {}", name),
            EvaluateError::FuncAlreadyDefined(name) => {
                write!(f, "Function already defined: {}", name)
            }
            EvaluateError::UnknownVar(name) => write!(f, "Unknown variable: {}", name),
            EvaluateError::VarAlreadyDefined(name) => {
                write!(f, "Variable already defined: {}", name)
            }
        }
    }
}

impl From<EvaluateError> for AspenError {
    fn from(value: EvaluateError) -> Self {
        AspenError::Evaluate(value)
    }
}
