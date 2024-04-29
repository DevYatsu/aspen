use crate::lexer::{Float, Integer, Token};

use super::{
    error::{AspenError, AspenResult},
    utils::TokenOption,
    Expr,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Str(&'a str),
    Int(Integer),
    Float(Float),
    Bool(bool),
    Nil,
}

/// Parses a value.
///
/// **NOTE: We assume the current token is a value!**
pub fn parse_value(token: Token<'_>) -> AspenResult<Value<'_>> {
    let value = match token {
        Token::Bool(b) => b.into(),
        Token::String(s) => s.into(),
        Token::Int(i) => i.into(),
        Token::Float(f) => f.into(),
        Token::Nil => Value::Nil,
        _ => return Err(AspenError::Expected("a valid <expr>".to_owned())),
    };

    Ok(value)
}

/// Parses a value or returns the found token.
///
/// **NOTE: We assume the current token is a value!**
pub fn parse_value_or_return_token(token: Token<'_>) -> AspenResult<TokenOption<'_, Value<'_>>> {
    let value: Value<'_> = match token {
        Token::Bool(b) => b.into(),
        Token::String(s) => s.into(),
        Token::Int(i) => i.into(),
        Token::Float(f) => f.into(),
        _ => return Ok(token.into()),
    };

    Ok(value.into())
}

crate::impl_from_for!(Value, Expr);

impl<'a> From<Value<'a>> for TokenOption<'a, Value<'a>> {
    fn from(value: Value<'a>) -> TokenOption<'a, Value<'a>> {
        TokenOption::Some(value)
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Value::Str(value)
    }
}
impl<'a> From<Integer> for Value<'a> {
    fn from(value: Integer) -> Self {
        Value::Int(value)
    }
}
impl<'a> From<Float> for Value<'a> {
    fn from(value: Float) -> Self {
        Value::Float(value)
    }
}
