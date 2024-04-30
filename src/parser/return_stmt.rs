use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_space},
    AspenParser, Container, Expr, Statement,
};
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Return<'a>(pub Vec<Box<Expr<'a>>>);
crate::impl_from_for!(Return, Statement);

impl<'a> Return<'a> {
    /// Parses a return statement.
    ///
    /// **NOTE: We assume "return" is already consumed by the lexer!**
    pub fn parse<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;
        let expr = Expr::parse(parser)?;
        Ok(Statement::Return(Return(vec![Box::new(expr)])))
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'a>) -> AspenResult<Expr<'a>> {
        let value = Expr::parse(parser)?;

        Ok(value)
    }
}
