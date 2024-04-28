use super::{
    error::{AspenError, AspenResult},
    parse_block,
    utils::{expect_space, next_jump_multispace, Block},
    Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct While<'a> {
    pub condition: Box<Expr<'a>>,
    pub body: Box<Block<'a>>,
}

impl<'s> While<'s> {
    /// Parses a while loop.
    ///
    /// **NOTE: We assume "while" is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;
        let condition = Box::new(Expr::parse(parser)?);

        let token = next_jump_multispace(parser)?;
        match token {
            Token::OpenBrace => (),
            _ => return Err(AspenError::Expected("a '{'".to_owned())),
        };

        let body = Box::new(parse_block(parser, Some(Token::CloseBrace))?);

        Ok(While { condition, body }.into())
    }
}

crate::impl_from_for!(While, Statement);
