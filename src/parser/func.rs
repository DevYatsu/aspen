use super::{
    comment::Comment,
    error::{AspenError, AspenResult},
    parse_block,
    utils::{next_jump_multispace, next_token, Block},
    Expr, Statement,
};
use crate::{
    lexer::Token,
    parser::{AspenParser, Container},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Func<'a> {
    pub name: &'a str,
    pub arguments: Container<Argument<'a>>,
    pub body: Box<Block<'a>>,
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
        let arguments = Func::parse_declaration_args(parser)?;
        let body = Box::new(parse_block(parser, Some(Token::CloseBrace))?);

        Ok(Func {
            name,
            arguments,
            body,
        }
        .into())
    }

    /// Parses arguments of a function call.
    ///
    /// **NOTE: We assume '(' was already consumed!**
    pub fn parse_call_args<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Vec<Box<Expr<'s>>>> {
        let mut args = vec![];
        let mut awaits_arg = true;

        loop {
            match next_jump_multispace(parser)? {
                Token::CloseParen => break,
                Token::LineComment(val) | Token::DocComment(val) | Token::MultiLineComment(val) => {
                    let start = parser.lexer.span().start;
                    let end = parser.lexer.span().end;
                    parser.add_comment(Comment::new(val, start, end))
                }
                token if awaits_arg => {
                    let expr = Expr::parse_with_token(parser, token)?;
                    args.push(Box::new(expr));
                    awaits_arg = false
                }
                Token::Range if !awaits_arg => {
                    // condition is sure to be true
                    if let Some(expr) = args.last_mut() {
                        Expr::modify_into_range(parser, expr)?;
                    }
                }
                Token::Dot if !awaits_arg => {
                    // condition is sure to be true
                    if let Some(expr) = args.last_mut() {
                        Expr::modify_into_obj_indexing(parser, expr)?;
                    }
                }
                Token::OpenBracket if !awaits_arg => {
                    // condition is sure to be true
                    if let Some(expr) = args.last_mut() {
                        Expr::modify_into_array_indexing(parser, expr)?;
                    }
                }
                Token::OpenParen if !awaits_arg => {
                    // condition is sure to be true
                    if let Some(expr) = args.last_mut() {
                        Expr::modify_into_fn_call(parser, expr)?;
                    }
                }
                Token::Comma if !awaits_arg => awaits_arg = true,
                _ => return Err(AspenError::Expected("a close parenthesis ')'".to_owned())),
            };
        }

        Ok(args)
    }

    /// Parses arguments of a function (when declaring it).
    ///
    /// **NOTE: We also parse the '{' which startes the block of the function**
    fn parse_declaration_args(
        parser: &mut AspenParser<'a>,
    ) -> AspenResult<Container<Argument<'a>>> {
        let mut args = vec![];
        let mut awaits_arg = true;

        loop {
            let token = next_jump_multispace(parser)?;

            match token {
                Token::OpenBrace => break,
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

impl<'a> Argument<'a> {
    pub fn new(value: &'a str, is_spread: bool) -> Self {
        Self { is_spread, value }
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
