use std::cmp::Ordering;

use super::{
    comment::Comment,
    error::{AspenError, AspenResult},
    func::Func,
    operator::BinaryOperator,
    utils::{expect_token, next_jump_multispace, next_token, TokenOption},
    value::{parse_value, parse_value_or_return_token, Value},
    Expr,
};
use crate::parser::{AspenParser, Token};
use hashbrown::HashMap;

impl<'a> Expr<'a> {
    /// Parses an expression.
    ///
    pub fn parse<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Expr<'s>> {
        let token = next_jump_multispace(parser)?;
        let expr = Self::parse_with_token(parser, token)?;

        Ok(expr)
    }

    /// Parses an expression.
    ///
    pub fn parse_with_token<'s>(
        parser: &mut AspenParser<'s>,
        token: Token<'s>,
    ) -> AspenResult<Expr<'s>> {
        let expr = match token {
            Token::OpenBracket => parse_array(parser)?.into(),
            Token::OpenBrace => parse_obj(parser)?.into(),
            Token::OpenParen => {
                let expr = Expr::Parenthesized(Box::new(Self::parse(parser)?));
                expect_token(parser, Token::CloseParen)?;
                expr
            }
            Token::Identifier(ident) => ident.into(),
            Token::SpreadOperator => {
                let next_token = next_token(parser)?;

                match next_token {
                    Token::Identifier(value) => Expr::SpeadId(value),
                    _ => {
                        return Err(AspenError::Expected(
                            "an identifier following the '...'".to_owned(),
                        ))
                    }
                }
            }
            token => parse_value(token)?.into(),
        };

        Ok(expr)
    }

    /// Parses an expression or returns the found token.
    ///
    pub fn parse_or_return_token<'s>(
        parser: &mut AspenParser<'s>,
    ) -> AspenResult<TokenOption<'s, Expr<'s>>> {
        let token = next_jump_multispace(parser)?;

        let expr_or_token: Expr<'s> = match token {
            Token::OpenBracket => parse_array(parser)?.into(),
            Token::OpenBrace => parse_obj(parser)?.into(),
            Token::OpenParen => {
                let expr = Expr::Parenthesized(Box::new(Self::parse(parser)?));
                expect_token(parser, Token::CloseParen)?;
                expr
            }
            Token::Identifier(ident) => ident.into(),
            Token::SpreadOperator => {
                let next_token = next_token(parser)?;

                match next_token {
                    Token::Identifier(value) => Expr::SpeadId(value),
                    _ => {
                        return Err(AspenError::Expected(
                            "an identifier following the '...'".to_owned(),
                        ))
                    }
                }
            }
            token => return Ok(parse_value_or_return_token(token)?.into()),
        };

        Ok(expr_or_token.into())
    }

    fn update_most_rhs(&mut self, value: Box<Expr<'a>>) {
        let mut expr = self;
        while let Expr::Binary { rhs, .. } = expr {
            expr = rhs;
        }
        *expr = *value;
    }

    fn add_func_call_to_most_rhs(&mut self, args: Vec<Expr<'a>>) {
        let mut expr = self;
        while let Expr::Binary { rhs, .. } = expr {
            expr = rhs;
        }
        *expr = Expr::FuncCall {
            callee: Box::new(expr.clone()),
            args,
        };
    }

    /// Parses a parenthesized expr.
    ///
    /// **NOTE: We assume '(' was already consumed!**
    pub fn parse_parenthesized<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Box<Expr<'s>>> {
        let token = next_jump_multispace(parser)?;
        let mut base_expr = Box::new(Expr::parse_with_token(parser, token)?);
        let mut bop: Option<BinaryOperator> = None; // bop for binary operator

        loop {
            let token = next_jump_multispace(parser)?;

            match token {
                token if bop.is_none() => match token {
                    Token::BinaryOperator(op) => {
                        bop = Some(op);
                    }
                    Token::OpenParen => match base_expr.as_mut() {
                        Expr::Id(_) => {
                            let args = Func::parse_call_args(parser)?;

                            base_expr = Box::new(
                                Expr::FuncCall {
                                    callee: base_expr,
                                    args,
                                }
                                .into(),
                            );
                        }
                        Expr::Binary { rhs, .. } => {
                            let args = Func::parse_call_args(parser)?;
                            rhs.add_func_call_to_most_rhs(args);
                        }
                        Expr::Assign { ref mut value, .. } => {
                            let args = Func::parse_call_args(parser)?;

                            match value.as_mut() {
                                Expr::Binary { rhs, .. } => {
                                    rhs.add_func_call_to_most_rhs(args);
                                }
                                Expr::Id(_) => {
                                    base_expr = Box::new(
                                        Expr::FuncCall {
                                            callee: base_expr,
                                            args,
                                        }
                                        .into(),
                                    );
                                }
                                _ => (),
                            }
                        }
                        _ => return Err(AspenError::Unknown("token '(' found".to_owned())),
                    },
                    Token::CloseParen => return Ok(base_expr),
                    _ => return Err(AspenError::Unknown(format!("token '{:?}' found", token))),
                },
                token if bop.is_some() => {
                    let right_expr = Expr::parse_with_token(parser, token)?;

                    match base_expr.as_mut() {
                        Expr::Binary { lhs, operator, rhs } => {
                            let result = operator
                                .get_precedence()
                                .cmp(&bop.as_ref().unwrap().get_precedence());
                            match result {
                                Ordering::Greater => {
                                    base_expr = Box::new(Expr::Binary {
                                        lhs: base_expr,
                                        operator: bop.take().unwrap(),
                                        rhs: Box::new(right_expr),
                                    });
                                }
                                Ordering::Equal | Ordering::Less => {
                                    base_expr = Expr::Binary {
                                        lhs: lhs.clone(),
                                        operator: operator.clone(),
                                        rhs: Box::new(Expr::Binary {
                                            lhs: rhs.clone(),
                                            operator: bop.take().unwrap(),
                                            rhs: Box::new(right_expr),
                                        }),
                                    }
                                    .into();
                                }
                            }
                        }
                        Expr::Assign {
                            target,
                            operator,
                            value,
                        } => {
                            base_expr = Expr::Assign {
                                target: target.clone(),
                                operator: operator.clone(),
                                value: Box::new(
                                    Expr::Binary {
                                        lhs: value.clone(),
                                        operator: bop.take().unwrap(),
                                        rhs: base_expr,
                                    }
                                    .into(),
                                ),
                            }
                            .into();
                        }
                        _ => {
                            base_expr = Expr::Binary {
                                lhs: base_expr,
                                operator: bop.take().unwrap(),
                                rhs: Box::new(right_expr),
                            }
                            .into();
                        }
                    };

                    bop = None;
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Parses an array.
///
/// **NOTE: We assume "[" was already consumed!**
fn parse_array<'s>(parser: &mut AspenParser<'s>) -> AspenResult<Vec<Box<Expr<'s>>>> {
    let mut arr = Vec::new();
    let mut awaits_comma = false;

    loop {
        let expr_or_token = Expr::parse_or_return_token(parser)?;

        match expr_or_token {
            TokenOption::Some(expr) if !awaits_comma => {
                arr.push(Box::new(expr.into()));
                awaits_comma = true;
            }
            TokenOption::Token(token) => match token {
                Token::CloseBracket => return Ok(arr),
                Token::Comma if awaits_comma => awaits_comma = false,
                Token::LineComment(val) | Token::DocComment(val) | Token::MultiLineComment(val) => {
                    let start = parser.lexer.span().start;
                    let end = parser.lexer.span().end;
                    parser.add_comment(Comment::new(val, start, end))
                }
                _ if awaits_comma => {
                    return Err(AspenError::Expected("a close bracket '}'".to_owned()))
                }
                _ if !awaits_comma => {
                    return Err(AspenError::Expected("a value <expr>".to_owned()))
                }
                _ => unreachable!("All cases are covered up there!"),
            },
            _ => return Err(AspenError::Expected("a close bracket '}'".to_owned())),
        };
    }
}

/// Parses an object.
///
/// **NOTE: We assume "{" was already consumed!**
fn parse_obj<'s>(parser: &mut AspenParser<'s>) -> AspenResult<HashMap<&'s str, Expr<'s>>> {
    let mut hash = HashMap::new();
    let mut key = None;
    let mut value = None;

    loop {
        let token = next_jump_multispace(parser)?;

        match token {
            // if value.is_some() then key is too!!!
            Token::ObjectKey(k) if key.is_none() => {
                key = Some(k);
            }
            Token::CloseBrace if value.is_some() => {
                hash.insert(key.take().unwrap(), value.take().unwrap());
                return Ok(hash);
            }
            Token::CloseBrace if key.is_none() => {
                return Ok(hash);
            }
            Token::Comma if value.is_some() => {
                hash.insert(key.take().unwrap(), value.take().unwrap());
            }
            Token::OpenParen => {
                let expr = Expr::Parenthesized(Box::new(Expr::parse(parser)?));
                expect_token(parser, Token::CloseParen)?;
                value = Some(expr)
            }
            Token::OpenBrace if key.is_some() => {
                let object = parse_obj(parser)?;
                value = Some(object.into());
            }
            Token::OpenBracket if key.is_some() => {
                let sub_array = parse_array(parser)?;
                value = Some(sub_array.into());
            }
            Token::Identifier(ident) if key.is_some() => {
                value = Some(ident.into());
            }
            Token::Identifier(ident) => {
                key = Some(ident);
                value = Some(ident.into());
            }
            Token::SpreadOperator if key.is_some() => return Err(AspenError::Expected(
                "an object or an array: either do {..<spread_variable>} or [...<spread_variable>]"
                    .to_owned(),
            )),
            Token::SpreadOperator => {
                let next_token = next_token(parser)?;

                match next_token {
                    Token::Identifier(ident) => {
                        key = Some(ident);
                        value = Some(Expr::SpeadId(ident));
                    }
                    _ => {
                        return Err(AspenError::Expected(
                            "an identifier following the '...'".to_owned(),
                        ))
                    }
                }
            }
            Token::LineComment(val) | Token::DocComment(val) | Token::MultiLineComment(val) => {
                let start = parser.lexer.span().start;
                let end = parser.lexer.span().end;
                parser.add_comment(Comment::new(val, start, end))
            }
            _ if key.is_some() => {
                value = Some(parse_value(token)?.into());
            }
            _ => return Err(AspenError::Expected("a valid <expr>".to_owned())),
        };
    }
}

impl<'a> From<Expr<'a>> for TokenOption<'a, Expr<'a>> {
    fn from(value: Expr<'a>) -> TokenOption<'a, Expr<'a>> {
        TokenOption::Some(value)
    }
}

impl<'a> From<TokenOption<'a, Value<'a>>> for TokenOption<'a, Expr<'a>> {
    fn from(value: TokenOption<'a, Value<'a>>) -> TokenOption<'a, Expr<'a>> {
        match value {
            TokenOption::Some(v) => TokenOption::Some(v.into()),
            TokenOption::Token(v) => TokenOption::Token(v),
        }
    }
}

impl<'a> Into<Expr<'a>> for Vec<Box<Expr<'a>>> {
    fn into(self) -> Expr<'a> {
        Expr::Array(self)
    }
}

impl<'a> Into<Expr<'a>> for HashMap<&'a str, Expr<'a>> {
    fn into(self) -> Expr<'a> {
        Expr::Object(self)
    }
}

impl<'a> Into<Expr<'a>> for &'a str {
    fn into(self) -> Expr<'a> {
        Expr::Id(self)
    }
}
