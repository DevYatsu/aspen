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
pub fn parse_value<'s>(token: Token<'s>) -> AspenResult<Value<'s>> {
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
pub fn parse_value_or_return_token<'s>(
    token: Token<'s>,
) -> AspenResult<TokenOption<'s, Value<'s>>> {
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
