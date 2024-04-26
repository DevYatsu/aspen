use crate::lexer::{AspenLexer, Token};

use super::error::{AspenError, AspenResult};

pub enum TokenOption<'a, T> {
    Some(T),
    Token(Token<'a>),
}

pub fn expect_space<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<()> {
    let token = next_token(lexer)?;

    match token {
        Token::Newline | Token::Spaces => (),
        _ => return Err(AspenError::ExpectedSpace),
    }

    Ok(())
}

pub fn expect_newline<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<()> {
    let token = next_token(lexer)?;
    println!("{:?}", token);

    match token {
        Token::Spaces => {
            let next_token = next_token(lexer)?;

            if next_token != Token::Newline {
                return Err(AspenError::ExpectedNewline);
            }
        }
        Token::Newline => (),
        _ => return Err(AspenError::ExpectedNewline),
    }

    Ok(())
}

pub fn next_token<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Token<'s>> {
    match lexer.next() {
        Some(result_token) => {
            let token = result_token?;

            Ok(token)
        }
        None => return Err(AspenError::Eof),
    }
}

pub fn next_jump_multispace<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Token<'s>> {
    loop {
        let token = next_token(lexer)?;

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
