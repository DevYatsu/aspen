use super::{
    error::{AspenError, AspenResult},
    utils::{expect_space, next_jump_multispace},
    Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'s> {
    pub variables: Variables<'s>,
    pub value: Box<Expr<'s>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Variables<'s> {
    Unique(&'s str),
    Destructuring(Vec<&'s str>),
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
        let mut variables = match next_jump_multispace(parser)? {
            Token::Identifier(name) => Variables::Unique(name),
            Token::OpenParen => {
                let mut variables = vec![];
                match next_jump_multispace(parser)? {
                    Token::Identifier(name) => variables.push(name),
                    _ => return Err(AspenError::expected(parser, "an identifier".to_owned())),
                }

                loop {
                    match next_jump_multispace(parser)? {
                        Token::Comma => {
                            match next_jump_multispace(parser)? {
                                Token::Identifier(name) => variables.push(name),
                                Token::CloseParen => break,
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

                Variables::Destructuring(variables)
            }
            _ => {
                return Err(AspenError::unknown(
                    parser,
                    format!("token '{}'", parser.lexer.slice()),
                ))
            }
        };

        let value = Box::new(Expr::parse(parser)?);

        Ok(Var { variables, value }.into())
    }
}

crate::impl_from_for!(Var, Statement);

impl<'s> Variables<'s> {
    pub fn len(&self) -> usize {
        match self {
            Variables::Unique(_) => 1,
            Variables::Destructuring(v) => v.len(),
        }
    }
}
