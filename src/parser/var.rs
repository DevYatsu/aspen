use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace, next_jump_space},
    Container, Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    pub variables: Vec<&'a str>,
    pub value: Box<Expr<'a>>,
}

impl<'a> Var<'a> {
    /// Parses an variable declaration.
    ///
    /// **NOTE: We assume "let" is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'a>) -> AspenResult<Statement<'a>> {
        expect_space(parser)?;

        Self::parse_after_comma(parser)
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'a>) -> AspenResult<Statement<'a>> {
        let mut variables = vec![];

        match next_jump_multispace(parser)? {
            Token::Identifier(name) => variables.push(name),
            Token::OpenBracket => {
                match next_jump_multispace(parser)? {
                    Token::Identifier(name) => variables.push(name),
                    _ => return Err(AspenError::Expected("an import value".to_owned())),
                }

                loop {
                    match next_jump_multispace(parser)? {
                        Token::Comma => {
                            match next_jump_multispace(parser)? {
                                Token::Identifier(name) => variables.push(name),
                                Token::CloseBracket => break,
                                _ => return Err(AspenError::Expected("an identifier".to_owned())),
                            };
                        }
                        Token::CloseBracket => break,
                        _ => return Err(AspenError::Expected("an identifier".to_owned())),
                    }
                }
            }
            _ => return Err(AspenError::Expected("an identifier".to_owned())),
        };

        let value = Box::new(Expr::parse(parser)?);

        Ok(Var { variables, value }.into())
    }
}

crate::impl_from_for!(Var, Statement);
