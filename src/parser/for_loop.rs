use super::{
    error::{AspenError, AspenResult},
    parse_block,
    utils::{expect_space, next_jump_multispace, Block},
    Expr, Statement,
};
use crate::parser::{AspenParser, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct For<'s> {
    pub args: Vec<&'s str>,
    pub indexed: Box<Expr<'s>>,
    pub body: Box<Block<'s>>,
}

impl<'s> For<'s> {
    /// Parses a for loop.
    ///
    /// **NOTE: We assume "for" is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Statement<'s>> {
        expect_space(parser)?;
        let args = Self::parse_args(parser)?;
        let (indexed, _) = Expr::parse_until(parser, &[Token::OpenBrace])?;
        let body = Box::new(parse_block(parser, Some(Token::CloseBrace))?);

        Ok(For {
            args,
            indexed,
            body,
        }
        .into())
    }

    /// Parses the arguments of a for loop.
    ///
    /// **NOTE: This function also parses the following "->" Token!**
    fn parse_args(parser: &mut AspenParser<'s>) -> AspenResult<Vec<&'s str>> {
        let mut args = vec![];
        let mut awaits_arg = true;

        loop {
            let token = next_jump_multispace(parser)?;

            match token {
                Token::Identifier(value) if awaits_arg => {
                    args.push(value);
                    awaits_arg = false
                }
                Token::In => break,
                Token::Comma if !awaits_arg => awaits_arg = true,
                _ => {
                    return Err(AspenError::expected(
                        parser,
                        "a valid argument or '->'".to_owned(),
                    ))
                }
            };
        }

        Ok(args)
    }
}

crate::impl_from_for!(For, Statement);
