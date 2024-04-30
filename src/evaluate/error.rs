use std::{error::Error, fmt};

use crate::parser::error::AspenError;

#[derive(Debug)]
pub enum ExecuteError {
    UnknownFunc(String),
    FuncAlreadyDefined(String),

    UnknownVar(String),
    VarAlreadyDefined(String),
}

impl Error for ExecuteError {}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecuteError::UnknownFunc(name) => write!(f, "Unknown function: {}", name),
            ExecuteError::FuncAlreadyDefined(name) => {
                write!(f, "Function already defined: {}", name)
            }
            ExecuteError::UnknownVar(name) => write!(f, "Unknown variable: {}", name),
            ExecuteError::VarAlreadyDefined(name) => {
                write!(f, "Variable already defined: {}", name)
            }
        }
    }
}

impl From<ExecuteError> for AspenError {
    fn from(value: ExecuteError) -> Self {
        AspenError::Execute(value)
    }
}
