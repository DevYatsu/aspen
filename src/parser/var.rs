use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace},
    Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'s> {
    pub variables: Vec<&'s str>,
    pub value: Box<Expr<'s>>,
}

impl<'s> Var<'s> {
    /// Parses an variable declaration.
    ///
    /// **NOTE: We assume "let" is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;

        Self::parse_after_comma(parser)
    }

    pub fn parse_after_comma(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        let mut variables = vec![];

        match next_jump_multispace(parser)? {
            Token::Identifier(name) => variables.push(name),
            Token::OpenParen => {
                match next_jump_multispace(parser)? {
                    Token::Identifier(name) => variables.push(name),
                    _ => return Err(AspenError::expected(parser, "an identifier".to_owned())),
                }

                loop {
                    match next_jump_multispace(parser)? {
                        Token::Comma => {
                            match next_jump_multispace(parser)? {
                                Token::Identifier(name) => variables.push(name),
                                Token::CloseBracket => break,
                                _ => {
                                    return Err(AspenError::expected(
                                        parser,
                                        "an identifier".to_owned(),
                                    ))
                                }
                            };
                        }
                        Token::CloseParen => break,
                        _ => return Err(AspenError::expected(parser, "an identifier".to_owned())),
                    }
                }
            }
            _ => return Err(AspenError::expected(parser, "an identifier".to_owned())),
        };

        let value = Box::new(Expr::parse(parser)?);

        Ok(Var { variables, value }.into())
    }
}

crate::impl_from_for!(Var, Statement);
