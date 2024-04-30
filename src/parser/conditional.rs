use std::borrow::BorrowMut;

use crate::lexer::Token;

use super::{
    error::{AspenError, AspenResult},
    parse_block,
    utils::{expect_space, expect_token, Block},
    AspenParser, Expr, Statement,
};

#[derive(Debug, Clone, PartialEq)]
pub struct If<'a> {
    pub condition: Box<Expr<'a>>,
    pub body: Block<'a>,
    pub other: Option<Box<IfOther<'a>>>,
}

crate::impl_from_for!(If, Statement);

#[derive(Debug, Clone, PartialEq)]
pub enum IfOther<'a> {
    If(If<'a>),
    Else(Block<'a>),
}

impl<'s> If<'s> {
    /// Parses an if statement.
    ///
    /// **NOTE: We assume "if" is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;
        let (condition, _) = Expr::parse_until(parser, &[Token::OpenBrace])?;
        let body = parse_block(parser, Some(Token::CloseBrace))?;

        Ok(If {
            condition,
            body,
            other: None,
        }
        .into())
    }

    pub fn parse_other(parser: &mut AspenParser<'s>) -> AspenResult<IfOther<'s>> {
        expect_space(parser)?;
        let (condition, _) = Expr::parse_until(parser, &[Token::OpenBrace])?;
        let body = parse_block(parser, Some(Token::CloseBrace))?;

        Ok(IfOther::If(If {
            condition,
            body,
            other: None,
        }))
    }

    pub fn add_other_at_if_end(&mut self, other_value: IfOther<'s>) -> AspenResult<()> {
        if let Some(if_other) = self.other.as_mut() {
            let mut current = if_other;
            loop {
                match current.borrow_mut() {
                    IfOther::If(boxed_if) => {
                        let If { ref mut other, .. } = *boxed_if;
                        if other.is_none() {
                            *other = Some(Box::new(other_value));
                            return Ok(());
                        } else {
                            current = other.as_mut().unwrap();
                        }
                    }
                    IfOther::Else(_) => {
                        return Err(AspenError::Unknown(
                            "'other' block found, the 'if' statement already possesses an 'else' clause"
                                .to_owned(),
                        ));
                    }
                }
            }
        } else {
            self.other = Some(Box::new(other_value));
            Ok(())
        }
    }

    pub fn parse_else(parser: &mut AspenParser<'s>) -> AspenResult<IfOther<'s>> {
        expect_token(parser, Token::OpenBrace)?;
        let body = parse_block(parser, Some(Token::CloseBrace))?;

        Ok(IfOther::Else(body))
    }
}
