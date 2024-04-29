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

    pub fn parse_several_or_complex_expr_or_newline(
        parser: &mut AspenParser<'a>,
        statements: &mut Container<Statement<'a>>,
    ) -> AspenResult<()> {
        loop {
            match next_jump_space(parser)? {
                Token::Newline => return Ok(()),
                Token::Comma => {
                    let expr = Return::parse_after_comma(parser)?;
                    let last = statements.last_mut().unwrap();

                    match last.as_mut() {
                        Statement::Return(Return(vec)) => vec.push(Box::new(expr)),
                        _ => unreachable!(),
                    }
                }
                Token::OpenParen => {
                    let stmt = statements.last_mut().unwrap();

                    // this if is inevitably true
                    if let Statement::Return(Return(vec)) = stmt.as_mut() {
                        let last_expr = vec.last_mut().unwrap();
                        Expr::modify_into_fn_call(parser, last_expr)?;
                    }
                }
                Token::BinaryOperator(bop) => {
                    let stmt = statements.last_mut().unwrap();

                    // this if is inevitably true
                    if let Statement::Return(Return(vec)) = stmt.as_mut() {
                        let right_expr = Expr::parse(parser)?;

                        let last_expr = vec.last_mut().unwrap();
                        Expr::modify_into_binary_op(last_expr, right_expr, bop)?;
                    }
                }
                _ => return Err(AspenError::ExpectedNewline),
            };
        }
    }
}
