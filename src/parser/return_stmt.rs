use super::{error::AspenResult, utils::expect_space, AspenParser, Expr, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Return<'s>(pub Box<Expr<'s>>);
crate::impl_from_for!(Return, Statement);

impl<'s> Return<'s> {
    /// Parses a return statement.
    ///
    /// **NOTE: We assume "return" is already consumed by the lexer!**
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        // we can return with '>>' token
        if parser.lexer.slice().len() != 2 {
            expect_space(parser)?;
        }
        let expr = Expr::parse(parser).map(Box::new)?;
        Ok(Statement::Return(Return(expr)))
    }
}
