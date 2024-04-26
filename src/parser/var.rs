use super::{
    error::{AspenError, AspenResult},
    expr::parse_expr,
    utils::{expect_space, next_jump_multispace},
    Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    pub name: &'a str,
    pub value: Expr<'a>,
}

/// Parses an variable declaration.
///
/// **NOTE: We assume "let" is already consumed by the parser!**
pub fn parse_var_stmt<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
    expect_space(parser)?;
    let token = next_jump_multispace(parser)?;

    let name = match token {
        Token::Identifier(name) => name,
        _ => return Err(AspenError::Expected("an import value".to_owned())),
    };

    expect_space(parser)?;
    let value = parse_expr(parser)?;

    Ok(Var { name, value }.into())
}

crate::impl_from_for!(Var, Statement);
