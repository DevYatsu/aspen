use super::{func::AspenFn, EvaluateResult};
use hashbrown::HashMap;
use rug::{float::OrdFloat, Float, Integer};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AspenValue<'a> {
    Nil,
    Str(String),
    Bool(bool),

    Error(String),

    Int(Integer),
    Float(OrdFloat),

    Array(Vec<AspenValue<'a>>),
    Object(HashMap<&'a str, AspenValue<'a>>),

    Range {
        start: Box<AspenValue<'a>>,
        end: Box<AspenValue<'a>>,
        step: Option<Box<AspenValue<'a>>>,
    },

    Func(AspenFn<'a>),

    RustBindFn {
        name: &'a str,
        code: fn(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>>,
    },
}

impl<'a> fmt::Display for AspenValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AspenValue::Nil => write!(f, "nil"),
            AspenValue::Str(s) => write!(f, "{}", s),
            AspenValue::Bool(b) => write!(f, "{}", b),
            AspenValue::Int(i) => write!(f, "{}", i.to_string()),
            AspenValue::Float(fl) => write!(f, "{}", Float::from(fl.to_owned()).to_string()),
            AspenValue::Error(s) => write!(f, "Err(\"{}\")", s),
            AspenValue::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            AspenValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}! {}", key, value)?;
                }
                write!(f, "}}")
            }
            AspenValue::Range { start, end, step } => {
                write!(f, "{}:{}", start, end)?;
                if let Some(step) = step {
                    write!(f, ":{}", step)?;
                }
                Ok(())
            }
            AspenValue::Func(func) => write!(f, "Func<{}>", func.name),
            AspenValue::RustBindFn { name, .. } => write!(f, "RustFunc<{}>", name),
        }
    }
}

impl<'a> From<String> for AspenValue<'a> {
    fn from(value: String) -> Self {
        AspenValue::Str(value)
    }
}
impl<'a> From<Integer> for AspenValue<'a> {
    fn from(value: Integer) -> Self {
        AspenValue::Int(value)
    }
}
impl<'a> From<()> for AspenValue<'a> {
    fn from(value: ()) -> Self {
        AspenValue::Nil
    }
}

impl<'a> From<Float> for AspenValue<'a> {
    fn from(value: Float) -> Self {
        AspenValue::Float(OrdFloat::from(value))
    }
}
impl<'a> From<OrdFloat> for AspenValue<'a> {
    fn from(value: OrdFloat) -> Self {
        AspenValue::Float(value)
    }
}
impl<'a> From<bool> for AspenValue<'a> {
    fn from(value: bool) -> Self {
        AspenValue::Bool(value)
    }
}
