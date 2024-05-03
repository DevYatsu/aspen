use super::{
    comment::Comment,
    error::{AspenError, AspenResult},
    parse_block,
    utils::{next_jump_multispace, next_token, Block},
    Expr, Statement,
};
use crate::{lexer::Token, parser::AspenParser};

#[derive(Debug, Clone, PartialEq)]
pub struct Func<'s> {
    pub name: &'s str,
    pub arguments: Vec<Argument<'s>>,
    pub body: Box<Block<'s>>,
}

crate::impl_from_for!(Func, Statement);

#[derive(Debug, Clone, PartialEq)]
pub struct Argument<'s> {
    pub is_spread: bool,
    pub identifier: &'s str,
    pub base_value: Option<Box<Expr<'s>>>,
}

impl<'s> Func<'s> {
    /// Parses a function declaration.
    ///
    /// **NOTE: We assume the function name is already consumed by the parser!**
    pub fn parse(parser: &mut AspenParser<'s>, name: &'s str) -> AspenResult<Statement<'s>> {
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
    pub fn parse_call_args(parser: &mut AspenParser<'s>) -> AspenResult<Vec<Box<Expr<'s>>>> {
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
                Token::PropagationOperator if !awaits_arg => {
                    // condition is sure to be true
                    if let Some(expr) = args.last_mut() {
                        Expr::modify_into_error_propagation(parser, expr)?;
                    }
                }
                Token::OpenParen if !awaits_arg => {
                    // condition is sure to be true
                    if let Some(expr) = args.last_mut() {
                        Expr::modify_into_fn_call(parser, expr)?;
                    }
                }
                Token::Comma if !awaits_arg => awaits_arg = true,
                _ => {
                    return Err(AspenError::expected(
                        parser,
                        "a close parenthesis ')'".to_owned(),
                    ))
                }
            };
        }

        Ok(args)
    }

    /// Parses arguments of a function (when declaring it).
    ///
    /// **NOTE: We also parse the '{' which startes the block of the function**
    fn parse_declaration_args(parser: &mut AspenParser<'s>) -> AspenResult<Vec<Argument<'s>>> {
        let mut args = Vec::new();
        let mut spread_count: u8 = 0;
        let mut awaits_arg = true;

        loop {
            let token = next_jump_multispace(parser)?;

            match token {
                Token::OpenBrace => break,
                // token is called range but it is just ':'
                Token::Range if !awaits_arg => {
                    if let Some(val) = args.last_mut() {
                        let Argument {
                            ref mut base_value, ..
                        } = val;

                        match base_value {
                            Some(_) => {
                                return Err(AspenError::unknown(
                                    parser,
                                    "token ':' found".to_owned(),
                                ))
                            }
                            None => {
                                let (expr, end_token) =
                                    Expr::parse_until(parser, &[Token::Comma, Token::OpenBrace])?;
                                *base_value = Some(expr);

                                if let Token::OpenBrace = end_token {
                                    break;
                                }

                                awaits_arg = true;
                            }
                        }
                    }
                }
                Token::Identifier(value) if awaits_arg => {
                    if spread_count > 0 {
                        return Err(AspenError::unknown(
                            parser,
                            format!("spread argument, a spread argument can only be defined at the end of the arguments list"),
                        ));
                    }

                    if args.iter().any(|arg| arg.identifier == value) {
                        return Err(AspenError::unknown(
                            parser,
                            format!(
                                "argument '{}', function already possesses such an identifier",
                                value
                            ),
                        ));
                    }

                    args.push(value.into());
                    awaits_arg = false
                }
                Token::SpreadOperator if awaits_arg => {
                    let next_token = next_token(parser)?;

                    if spread_count > 0 {
                        return Err(AspenError::unknown(
                            parser,
                            format!("spread argument, a function can only have 1 spread argument"),
                        ));
                    }

                    match next_token {
                        Token::Identifier(value) => {
                            if args.iter().any(|arg| arg.identifier == value) {
                                return Err(AspenError::unknown(
                                    parser,
                                    format!(
                                    "argument '{}', function already possesses such an identifier",
                                    value
                                ),
                                ));
                            }
                            args.push((value, true).into())
                        }
                        _ => {
                            return Err(AspenError::expected(
                                parser,
                                "an identifier following the '...'".to_owned(),
                            ))
                        }
                    };

                    spread_count += 1;
                    awaits_arg = false
                }
                Token::Comma if !awaits_arg => awaits_arg = true,
                _ => {
                    return Err(AspenError::expected(
                        parser,
                        "a valid function argument or '{'".to_owned(),
                    ))
                }
            };
        }

        Ok(args)
    }
}

impl<'s> Argument<'s> {
    pub fn new(identifier: &'s str, is_spread: bool) -> Self {
        Self {
            is_spread,
            identifier,
            base_value: None,
        }
    }
}

impl<'s> From<(&'s str, bool)> for Argument<'s> {
    fn from(value: (&'s str, bool)) -> Self {
        Argument::new(value.0, value.1)
    }
}
impl<'s> From<&'s str> for Argument<'s> {
    fn from(value: &'s str) -> Self {
        Argument::new(value, false)
    }
}
