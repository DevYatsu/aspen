use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace, next_jump_space},
    AspenParser, Container, Statement,
};
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub name: &'a str,
}

crate::impl_from_for!(Import, Statement);

impl<'a> Import<'a> {
    /// Parses an import statement.
    ///
    /// **NOTE: We assume "import" is already consumed by the lexer!**
    pub fn parse<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;

        match next_jump_multispace(parser)? {
            Token::String(name) => Ok(Import { name }.into()),
            _ => Err(AspenError::Expected("an import value".to_owned())),
        }
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'a>) -> AspenResult<Statement<'a>> {
        match next_jump_multispace(parser)? {
            Token::String(name) => Ok(Import { name }.into()),
            _ => Err(AspenError::Expected("an import value".to_owned())),
        }
    }
}
