use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace, next_jump_space},
    Container, Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    pub name: &'a str,
    pub value: Box<Expr<'a>>,
}

impl<'a> Var<'a> {
    /// Parses an variable declaration.
    ///
    /// **NOTE: We assume "let" is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'a>) -> AspenResult<Statement<'a>> {
        expect_space(parser)?;
        let token = next_jump_multispace(parser)?;

        let name = match token {
            Token::Identifier(name) => name,
            _ => return Err(AspenError::Expected("an import value".to_owned())),
        };

        expect_space(parser)?;
        let value = Box::new(Expr::parse(parser)?);

        Ok(Var { name, value }.into())
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'a>) -> AspenResult<Statement<'a>> {
        let token = next_jump_multispace(parser)?;

        let name = match token {
            Token::Identifier(name) => name,
            _ => return Err(AspenError::Expected("a variable declaration".to_owned())),
        };

        expect_space(parser)?;
        let value = Box::new(Expr::parse(parser)?);

        Ok(Var { name, value }.into())
    }

    pub fn parse_several_vars_or_complex_expr_or_newline(
        parser: &mut AspenParser<'a>,
        statements: &mut Container<Statement<'a>>,
    ) -> AspenResult<()> {
        loop {
            let next = next_jump_space(parser)?;
            match next {
                Token::Newline => return Ok(()),
                Token::Comma => {
                    let stmt = Var::parse_after_comma(parser)?;
                    statements.push(Box::new(stmt));
                }
                Token::OpenParen => {
                    let stmt = statements.last_mut().unwrap();

                    // this if is inevitably true
                    if let Statement::Var(var) = stmt.as_mut() {
                        let Var { value, .. } = var;
                        Expr::modify_into_fn_call(parser, value)?;
                    }
                }
                Token::BinaryOperator(bop) => {
                    let stmt = statements.last_mut().unwrap();

                    // this if is inevitably true
                    if let Statement::Var(var) = stmt.as_mut() {
                        let right_expr = Expr::parse(parser)?;

                        let Var { value, .. } = var;
                        Expr::modify_into_binary_op(value, right_expr, bop)?;
                    }
                }
                _ => return Err(AspenError::ExpectedNewline),
            };
        }
    }
}

crate::impl_from_for!(Var, Statement);
