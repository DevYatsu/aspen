use crate::lexer::{AspenLexer, Token};

use super::error::{AspenError, AspenResult};

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

    match token {
        Token::Spaces => {
            if next_token(lexer)? != Token::Newline {
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

pub fn next_while_space<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Token<'s>> {
    loop {
        let token = next_token(lexer)?;

        match token {
            Token::Newline | Token::Spaces => (),
            _ => return Ok(token),
        }
    }
}
