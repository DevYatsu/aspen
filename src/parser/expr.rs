use hashbrown::HashMap;

use crate::lexer::{AspenLexer, Token};

use super::{
    error::{AspenError, AspenResult},
    utils::{next_jump_multispace, TokenOption},
    value::{parse_value, parse_value_or_return_token, Value},
    Expr,
};

/// Parses an expression.
///
pub fn parse_expr<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Expr<'s>> {
    let token = next_jump_multispace(lexer)?;

    let expr = match token {
        Token::OpenBracket => parse_array(lexer)?.into(),
        Token::OpenBrace => parse_obj(lexer)?.into(),
        Token::Identifier(ident) => ident.into(),
        token => parse_value(token)?.into(),
    };

    Ok(expr)
}

/// Parses an expression or returns the found token.
///
pub fn parse_expr_or_return_token<'s>(
    lexer: &mut AspenLexer<'s>,
) -> AspenResult<TokenOption<'s, Expr<'s>>> {
    let token = next_jump_multispace(lexer)?;

    let expr_or_token: Expr<'s> = match token {
        Token::OpenBracket => parse_array(lexer)?.into(),
        Token::OpenBrace => parse_obj(lexer)?.into(),
        Token::Identifier(ident) => ident.into(),
        token => return Ok(parse_value_or_return_token(token)?.into()),
    };

    Ok(expr_or_token.into())
}

/// Parses an array.
///
/// **NOTE: We assume "[" was already consumed!**
pub fn parse_array<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Vec<Box<Expr<'s>>>> {
    let mut arr = Vec::new();
    let mut awaits_comma = false;
    let mut awaits_value = true;

    loop {
        let expr_or_token = parse_expr_or_return_token(lexer)?;

        match expr_or_token {
            TokenOption::Some(expr) if !awaits_comma => {
                arr.push(Box::new(expr.into()));
                awaits_value = false;
            }
            TokenOption::Token(token) => match token {
                Token::CloseBracket => return Ok(arr),
                Token::Comma if awaits_comma => awaits_value = true,
                _ if awaits_value => {
                    arr.push(Box::new(parse_value(token)?.into()));
                    awaits_value = false;
                }
                _ => return Err(AspenError::Expected("a valid <expr>".to_owned())),
            },
            _ => return Err(AspenError::Expected("a valid <expr>".to_owned())),
            // Token::OpenBrace if !awaits_comma => {
            //     let object = parse_obj(lexer)?;
            //     arr.push(Box::new(object.into()));
            //     awaits_value = false;
            // }
            // Token::OpenBracket if !awaits_comma => {
            //     let sub_array = parse_array(lexer)?;
            //     arr.push(Box::new(sub_array.into()));
            //     awaits_value = false;
            // }
            // Token::Identifier(ident) => {
            //     arr.push(Box::new(ident.into()));
            //     awaits_value = false;
            // }
        };
        awaits_comma = !awaits_value;
    }
}

/// Parses an object.
///
/// **NOTE: We assume "{" was already consumed!**
pub fn parse_obj<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<HashMap<&'s str, Expr<'s>>> {
    let mut hash = HashMap::new();
    let mut key = None;
    let mut value = None;

    loop {
        let token = next_jump_multispace(lexer)?;

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
                let object = parse_obj(lexer)?;
                value = Some(object.into());
            }
            Token::OpenBracket if key.is_some() => {
                let sub_array = parse_array(lexer)?;
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
