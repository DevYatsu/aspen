use super::func::AspenFn;
use hashbrown::HashMap;
use rug::{float::OrdFloat, Integer};

#[derive(Debug, Clone, PartialEq)]
pub enum AspenValue<'a> {
    Nil,
    Str(String),
    Bool(bool),

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
}

use std::fmt;

impl<'a> fmt::Display for AspenValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AspenValue::Nil => write!(f, "nil"),
            AspenValue::Str(s) => write!(f, "\"{}\"", s),
            AspenValue::Bool(b) => write!(f, "{}", b),
            AspenValue::Int(i) => write!(f, "{}", i),
            AspenValue::Float(fl) => write!(f, "{:?}", fl),
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
        }
    }
}
