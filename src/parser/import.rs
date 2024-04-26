use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace},
    AspenParser, Statement,
};
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub name: &'a str,
}

crate::impl_from_for!(Import, Statement);

/// Parses an import statement.
///
/// **NOTE: We assume "import" is already consumed by the lexer!**
pub fn parse_import_stmt<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
    expect_space(parser)?;
    let token = next_jump_multispace(parser)?;

    match token {
        Token::String(name) => Ok(Import { name }.into()),
        _ => Err(AspenError::Expected("an import value".to_owned())),
    }
}
