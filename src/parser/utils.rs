use super::error::{AspenError, AspenResult};
use crate::parser::{AspenParser, Container, Statement, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenOption<'a, T> {
    Some(T),
    Token(Token<'a>),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block<'a> {
    statements: Container<Statement<'a>>,
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

/// Retrieves the next token while jumping spaces and newline tokens. To jump only spaces see [`next_jump_space`].
pub fn next_jump_multispace<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Token<'s>> {
    loop {
        let token = next_token(parser)?;

        match token {
            Token::Newline | Token::Spaces => (),
            _ => return Ok(token),
        }
    }
}

/// Retrieves the next token while jumping spaces tokens. To jump newline tokens as well see [`next_jump_multispace`].
pub fn next_jump_space<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Token<'s>> {
    loop {
        let token = next_token(parser)?;

        match token {
            Token::Spaces => (),
            _ => return Ok(token),
        }
    }
}

impl<'a> Block<'a> {
    pub fn new(statements: Container<Statement<'a>>) -> Self {
        Self { statements }
    }
    pub fn statements(&self) -> Container<Statement<'a>> {
        self.statements.clone()
    }
    pub fn add_statement(&mut self, stmt: Statement<'a>) {
        self.statements.push(Box::new(stmt))
    }
    pub fn extend_statements(&mut self, statements: Container<Statement<'a>>) {
        self.statements.extend(statements)
    }
}

impl<'a, T> From<Token<'a>> for TokenOption<'a, T> {
    fn from(value: Token<'a>) -> TokenOption<'a, T> {
        TokenOption::Token(value)
    }
}
