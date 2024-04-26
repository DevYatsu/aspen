use hashbrown::HashMap;

use crate::parser::{AspenParser, Token};

use super::{
    error::{AspenError, AspenResult},
    utils::{next_jump_multispace, TokenOption},
    value::{parse_value, parse_value_or_return_token, Value},
    Expr,
};

/// Parses an expression.
///
pub fn parse_expr<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Expr<'s>> {
    let token = next_jump_multispace(parser)?;

    let expr = match token {
        Token::OpenBracket => parse_array(parser)?.into(),
        Token::OpenBrace => parse_obj(parser)?.into(),
        Token::Identifier(ident) => ident.into(),
        token => parse_value(token)?.into(),
    };

    Ok(expr)
}

/// Parses an expression or returns the found token.
///
pub fn parse_expr_or_return_token<'s>(
    parser: &mut AspenParser<'s>,
) -> AspenResult<TokenOption<'s, Expr<'s>>> {
    let token = next_jump_multispace(parser)?;

    let expr_or_token: Expr<'s> = match token {
        Token::OpenBracket => parse_array(parser)?.into(),
        Token::OpenBrace => parse_obj(parser)?.into(),
        Token::Identifier(ident) => ident.into(),
        token => return Ok(parse_value_or_return_token(token)?.into()),
    };

    Ok(expr_or_token.into())
}

/// Parses an array.
///
/// **NOTE: We assume "[" was already consumed!**
fn parse_array<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Vec<Box<Expr<'s>>>> {
    let mut arr = Vec::new();
    let mut awaits_comma = false;

    loop {
        let expr_or_token = parse_expr_or_return_token(parser)?;

        match expr_or_token {
            TokenOption::Some(expr) if !awaits_comma => {
                arr.push(Box::new(expr.into()));
                awaits_comma = true;
            }
            TokenOption::Token(token) => match token {
                Token::CloseBracket => return Ok(arr),
                Token::Comma if awaits_comma => awaits_comma = false,
                _ if awaits_comma => {
                    return Err(AspenError::Expected("a close bracket '}'".to_owned()))
                }
                _ if !awaits_comma => {
                    return Err(AspenError::Expected("a value <expr>".to_owned()))
                }
                _ => unreachable!("All cases are covered up there!"),
            },
            _ => return Err(AspenError::Expected("a close bracket '}'".to_owned())),
        };
    }
}

/// Parses an object.
///
/// **NOTE: We assume "{" was already consumed!**
fn parse_obj<'s>(parser: &mut AspenParser<'s>) -> AspenResult<HashMap<&'s str, Expr<'s>>> {
    let mut hash = HashMap::new();
    let mut key = None;
    let mut value = None;

    loop {
        let token = next_jump_multispace(parser)?;

        match token {
            // if value.is_some() then key is too!!!
            Token::ObjectKey(k) if key.is_none() => {
                key = Some(k);
            }
            Token::CloseBrace if value.is_some() => {
                hash.insert(key.take().unwrap(), value.take().unwrap());
                return Ok(hash);
            }
            Token::CloseBrace if key.is_none() => {
                return Ok(hash);
            }
            Token::Comma if value.is_some() => {
                hash.insert(key.take().unwrap(), value.take().unwrap());
            }
            Token::OpenBrace if key.is_some() => {
                let object = parse_obj(parser)?;
                value = Some(object.into());
            }
            Token::OpenBracket if key.is_some() => {
                let sub_array = parse_array(parser)?;
                value = Some(sub_array.into());
            }
            Token::Identifier(ident) if key.is_some() => {
                value = Some(ident.into());
            }
            _ if key.is_some() => {
                value = Some(parse_value(token)?.into());
            }
            _ => return Err(AspenError::Expected("a valid <expr>".to_owned())),
        };
    }
}

impl<'a> From<Expr<'a>> for TokenOption<'a, Expr<'a>> {
    fn from(value: Expr<'a>) -> TokenOption<'a, Expr<'a>> {
        TokenOption::Some(value)
    }
}

impl<'a> From<TokenOption<'a, Value<'a>>> for TokenOption<'a, Expr<'a>> {
    fn from(value: TokenOption<'a, Value<'a>>) -> TokenOption<'a, Expr<'a>> {
        match value {
            TokenOption::Some(v) => TokenOption::Some(v.into()),
            TokenOption::Token(v) => TokenOption::Token(v),
        }
    }
}

impl<'a> Into<Expr<'a>> for Vec<Box<Expr<'a>>> {
    fn into(self) -> Expr<'a> {
        Expr::Array(self)
    }
}

impl<'a> Into<Expr<'a>> for HashMap<&'a str, Expr<'a>> {
    fn into(self) -> Expr<'a> {
        Expr::Object(self)
    }
}

impl<'a> Into<Expr<'a>> for &'a str {
    fn into(self) -> Expr<'a> {
        Expr::Id(self)
    }
}
