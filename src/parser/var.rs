use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace, next_jump_space},
    Container, Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    pub name: &'a str,
    pub value: Expr<'a>,
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
        let value = Expr::parse(parser)?;

        Ok(Var { name, value }.into())
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'a>) -> AspenResult<Statement<'a>> {
        let token = next_jump_multispace(parser)?;

        let name = match token {
            Token::Identifier(name) => name,
            _ => return Err(AspenError::Expected("an import value".to_owned())),
        };

        expect_space(parser)?;
        let value = Expr::parse(parser)?;

        Ok(Var { name, value }.into())
    }

    pub fn parse_several_vars_or_newline(
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
                _ => return Err(AspenError::ExpectedNewline),
            };
        }
    }
}

crate::impl_from_for!(Var, Statement);
