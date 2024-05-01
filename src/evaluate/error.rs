use std::{error::Error, fmt};

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
