use super::{error::AspenResult, utils::expect_space, AspenParser, Expr, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Return<'s>(pub Vec<Box<Expr<'s>>>);
crate::impl_from_for!(Return, Statement);

impl<'s> Return<'s> {
    /// Parses a return statement.
    ///
    /// **NOTE: We assume "return" is already consumed by the lexer!**
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;
        let expr = Expr::parse(parser)?;
        Ok(Statement::Return(Return(vec![Box::new(expr)])))
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'s>) -> AspenResult<Expr<'s>> {
        let value = Expr::parse(parser)?;

        Ok(value)
    }
}
