use rug::float::OrdFloat;

use crate::lexer::{Integer, Token};

use super::{
    error::{AspenError, AspenResult},
    utils::TokenOption,
    AspenParser, Expr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value<'s> {
    Str(&'s str),
    Int(Integer),
    Float(OrdFloat),
    Bool(bool),
    Nil,
}

/// Parses a value.
///
/// **NOTE: We assume the current token is a value!**
pub fn parse_value<'s>(parser: &mut AspenParser<'s>, token: Token<'s>) -> AspenResult<Value<'s>> {
    let value = match token {
        Token::Bool(b) => b.into(),
        Token::String(s) => s.into(),
        Token::Int(i) => i.into(),
        Token::Float(f) => f.into(),
        Token::Nil => Value::Nil,
        _ => return Err(AspenError::expected(parser, "a valid <expr>".to_owned())),
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

impl<'s> From<Value<'s>> for TokenOption<'s, Value<'s>> {
    fn from(value: Value<'s>) -> TokenOption<'s, Value<'s>> {
        TokenOption::Some(value)
    }
}

impl<'s> From<bool> for Value<'s> {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
impl<'s> From<&'s str> for Value<'s> {
    fn from(value: &'s str) -> Self {
        Value::Str(value)
    }
}
impl<'s> From<Integer> for Value<'s> {
    fn from(value: Integer) -> Self {
        Value::Int(value)
    }
}
impl<'s> From<OrdFloat> for Value<'s> {
    fn from(value: OrdFloat) -> Self {
        Value::Float(value)
    }
}
