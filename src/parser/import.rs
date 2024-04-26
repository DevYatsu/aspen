use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace},
    Statement,
};
use crate::lexer::{AspenLexer, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Import<'a> {
    pub name: &'a str,
}

crate::impl_from_for!(Import, Statement);

/// Parses an import statement.
///
/// **NOTE: We assume "import" is already consumed by the lexer!**
pub fn parse_import_stmt<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Statement<'s>> {
    expect_space(lexer)?;
    let token = next_jump_multispace(lexer)?;

    match token {
        Token::String(name) => Ok(Import { name }.into()),
        _ => Err(AspenError::Expected("an import value".to_owned())),
    }
}
