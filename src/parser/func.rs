use super::{
    error::{AspenError, AspenResult},
    parse_block,
    utils::{expect_space, next_jump_multispace, next_token, Block},
    Statement,
};
use crate::{
    lexer::Token,
    parser::{AspenParser, Container},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Func<'a> {
    name: &'a str,
    arguments: Container<Argument<'a>>,
    body: Block<'a>,
}

crate::impl_from_for!(Func, Statement);

#[derive(Debug, Clone, PartialEq)]
pub struct Argument<'a> {
    pub is_spread: bool,
    pub value: &'a str,
}

impl<'a> Func<'a> {
    /// Parses a function declaration.
    ///
    /// **NOTE: We assume the function name is already consumed by the parser!**
    pub fn parse<'s>(parser: &mut AspenParser<'s>, name: &'s str) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;
        let arguments = Argument::parse_fn_args(parser)?;
        let body = parse_block(parser, Some(Token::CloseBrace))?;

        Ok(Func {
            name,
            arguments,
            body,
        }
        .into())
    }
}

impl<'a> Argument<'a> {
    pub fn new(value: &'a str, is_spread: bool) -> Self {
        Self { is_spread, value }
    }

    /// Parses arguments of a function.
    ///
    /// **NOTE: We also parse the '{' which startes the block of the function**
    fn parse_fn_args<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Container<Argument<'s>>> {
        let mut args = vec![];
        let mut awaits_arg = true;

        loop {
            let token = next_jump_multispace(parser)?;

            match token {
                Token::Identifier(value) if awaits_arg => {
                    args.push(Box::new(value.into()));
                    awaits_arg = false
                }

                Token::SpreadOperator if awaits_arg => {
                    let next_token = next_token(parser)?;

                    match next_token {
                        Token::Identifier(value) => args.push(Box::new((value, true).into())),
                        _ => {
                            return Err(AspenError::Expected(
                                "an identifier following the '...'".to_owned(),
                            ))
                        }
                    };

                    awaits_arg = false
                }
                Token::OpenBrace => break,
                Token::Comma if !awaits_arg => awaits_arg = true,
                _ => {
                    return Err(AspenError::Expected(
                        "a valid function argument or '{'".to_owned(),
                    ))
                }
            };
        }

        Ok(args)
    }
}

impl<'a> From<(&'a str, bool)> for Argument<'a> {
    fn from(value: (&'a str, bool)) -> Self {
        Argument::new(value.0, value.1)
    }
}
impl<'a> From<&'a str> for Argument<'a> {
    fn from(value: &'a str) -> Self {
        Argument::new(value, false)
    }
}
