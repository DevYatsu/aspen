use super::{
    error::{AspenError, AspenResult},
    expr::parse_expr,
    utils::{expect_space, next_while_space},
    Expr, Statement,
};
use crate::lexer::{AspenLexer, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    pub name: &'a str,
    pub value: Expr<'a>,
}

/// Parses an variable declaration.
///
/// **NOTE: We assume "let" is already consumed by the lexer!**
pub fn parse_var_stmt<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Statement<'s>> {
    expect_space(lexer)?;
    let token = next_while_space(lexer)?;

    let name = match token {
        Token::Identifier(name) => name,
        _ => return Err(AspenError::Expected("an import value".to_owned())),
    };

    expect_space(lexer)?;
    let value = parse_expr(lexer)?;

    Ok(Var { name, value }.into())
}

crate::impl_from_for!(Var, Statement);
