#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AspenType {
    Int,
    Float,
    Number,

    Bool,
    String,

    Array,
    Object,

    Range,

    Func,
    Nil,
}

use std::fmt;

use super::AspenValue;

impl fmt::Display for AspenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AspenType::Int => write!(f, "Int"),
            AspenType::Float => write!(f, "Float"),
            AspenType::Number => write!(f, "Number"),
            AspenType::Bool => write!(f, "Bool"),
            AspenType::String => write!(f, "String"),
            AspenType::Array => write!(f, "Array"),
            AspenType::Object => write!(f, "Object"),
            AspenType::Range => write!(f, "Range"),
            AspenType::Func => write!(f, "Func"),
            AspenType::Nil => write!(f, "Nil"),
            AspenType::Func => write!(f, "Func"),
        }
    }
}

impl<'a> From<AspenValue<'a>> for AspenType {
    fn from(value: AspenValue<'a>) -> Self {
        match value {
            AspenValue::Nil => AspenType::Nil,
            AspenValue::Str(_) => AspenType::String,
            AspenValue::Bool(_) => AspenType::Bool,
            AspenValue::Int(_) => AspenType::Int,
            AspenValue::Float(_) => AspenType::Float,
            AspenValue::Array(_) => AspenType::Array,
            AspenValue::Object(_) => AspenType::Object,
            AspenValue::Range { .. } => AspenType::Range,
            AspenValue::Func(_) => AspenType::Func,
            AspenValue::RustBindFn { .. } => AspenType::Func,
        }
    }
}

impl<'a, T: AsRef<AspenValue<'a>>> From<T> for AspenType {
    fn from(value: T) -> Self {
        match value.as_ref() {
            AspenValue::Nil => AspenType::Nil,
            AspenValue::Str(_) => AspenType::String,
            AspenValue::Bool(_) => AspenType::Bool,
            AspenValue::Int(_) => AspenType::Int,
            AspenValue::Float(_) => AspenType::Float,
            AspenValue::Array(_) => AspenType::Array,
            AspenValue::Object(_) => AspenType::Object,
            AspenValue::Range { .. } => AspenType::Range,
            AspenValue::Func(_) => AspenType::Func,
            AspenValue::RustBindFn { .. } => AspenType::Func,
        }
    }
}
