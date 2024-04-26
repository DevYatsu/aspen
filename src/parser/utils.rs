use crate::parser::{AspenParser, Token};

use super::error::{AspenError, AspenResult};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenOption<'a, T> {
    Some(T),
    Token(Token<'a>),
}

pub fn expect_space<'s>(parser: &mut AspenParser<'s>) -> AspenResult<()> {
    let token = next_token(parser)?;

    match token {
        Token::Newline | Token::Spaces => (),
        _ => return Err(AspenError::ExpectedSpace),
    }

    Ok(())
}

pub fn expect_newline<'s>(parser: &mut AspenParser<'s>) -> AspenResult<()> {
    let token = next_token(parser)?;

    match token {
        Token::Spaces => {
            let next_token = next_token(parser)?;

            if next_token != Token::Newline {
                return Err(AspenError::ExpectedNewline);
            }
        }
        Token::Newline => (),
        _ => return Err(AspenError::ExpectedNewline),
    }

    Ok(())
}

pub fn next_token<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Token<'s>> {
    match parser.lexer.next() {
        Some(result_token) => {
            let token = result_token?;

            Ok(token)
        }
        None => return Err(AspenError::Eof),
    }
}

pub fn next_jump_multispace<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Token<'s>> {
    loop {
        let token = next_token(parser)?;

        match token {
            Token::Newline | Token::Spaces => (),
            _ => return Ok(token),
        }
    }
}

impl<'a, T> From<Token<'a>> for TokenOption<'a, T> {
    fn from(value: Token<'a>) -> TokenOption<'a, T> {
        TokenOption::Token(value)
    }
}
