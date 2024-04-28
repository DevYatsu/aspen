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
        let token = next_jump_multispace(parser)?;

        match token {
            Token::String(name) => Ok(Import { name }.into()),
            _ => Err(AspenError::Expected("an import value".to_owned())),
        }
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'a>) -> AspenResult<Statement<'a>> {
        let token = next_jump_multispace(parser)?;

        match token {
            Token::String(name) => Ok(Import { name }.into()),
            _ => Err(AspenError::Expected("an import value".to_owned())),
        }
    }

    pub fn parse_several_or_newline(
        parser: &mut AspenParser<'a>,
        statements: &mut Container<Statement<'a>>,
    ) -> AspenResult<()> {
        loop {
            let next = next_jump_space(parser)?;
            match next {
                Token::Newline => return Ok(()),
                Token::Comma => {
                    let stmt = Import::parse_after_comma(parser)?;
                    statements.push(Box::new(stmt));
                }
                _ => return Err(AspenError::ExpectedNewline),
            };
        }
    }
}
