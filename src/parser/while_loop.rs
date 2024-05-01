use super::{
    error::AspenResult,
    parse_block,
    utils::{expect_space, Block},
    Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct While<'s> {
    pub condition: Box<Expr<'s>>,
    pub body: Box<Block<'s>>,
}
crate::impl_from_for!(While, Statement);

impl<'s> While<'s> {
    /// Parses a while loop.
    ///
    /// **NOTE: We assume "while" is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;
        let (condition, _) = Expr::parse_until(parser, &[Token::OpenBrace])?;
        let body = Box::new(parse_block(parser, Some(Token::CloseBrace))?);

        Ok(While { condition, body }.into())
    }
}
