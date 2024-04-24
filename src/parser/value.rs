use rug::{Float, Integer};

use crate::lexer::{AspenLexer, Token};

use super::{
    error::{AspenError, AspenResult},
    utils::next_while_space,
    Expr,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Str(&'a str),
    Int(Integer),
    Float(Float),
    Bool(bool),
}

/// Parses a value.
///
/// **NOTE: We assume the next token is a value!**
pub fn parse_value<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Value<'s>> {
    let token = next_while_space(lexer)?;

    let value = match token {
        Token::Bool(b) => b.into(),
        Token::String(s) => s.into(),
        Token::Int(i) => i.into(),
        Token::Float(f) => f.into(),
        _ => {
            return Err(AspenError::ExpectedString(
                "Expected an import value".to_owned(),
            ))
        }
    };

    Ok(value)
}

crate::impl_from_for!(Value, Expr);

impl<'a> Into<Value<'a>> for bool {
    fn into(self) -> Value<'a> {
        Value::Bool(self)
    }
}
impl<'a> Into<Value<'a>> for &'a str {
    fn into(self) -> Value<'a> {
        Value::Str(self)
    }
}
impl<'a> Into<Value<'a>> for Integer {
    fn into(self) -> Value<'a> {
        Value::Int(self)
    }
}
impl<'a> Into<Value<'a>> for Float {
    fn into(self) -> Value<'a> {
        Value::Float(self)
    }
}
