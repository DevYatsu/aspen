use super::{
    comment::Comment,
    error::{AspenError, AspenResult},
    func::Func,
    operator::BinaryOperator,
    utils::{expect_token, next_jump_multispace, next_token, TokenOption},
    value::{parse_value, Value},
    Expr,
};
use crate::parser::{AspenParser, Token};
use hashbrown::HashMap;
use std::cmp::Ordering;

impl<'s> Expr<'s> {
    /// Parses an expression.
    ///
    pub fn parse(parser: &mut AspenParser<'s>) -> AspenResult<Expr<'s>> {
        let token = next_jump_multispace(parser)?;
        let expr = Self::parse_with_token(parser, token)?;

        Ok(expr)
    }

    /// Parses an expression.
    ///
    pub fn parse_with_token(
        parser: &mut AspenParser<'s>,
        token: Token<'s>,
    ) -> AspenResult<Expr<'s>> {
        let expr = match token {
            Token::OpenBracket => parse_array(parser)?.into(),
            Token::OpenBrace => parse_obj(parser)?.into(),
            Token::OpenParen => Expr::parse_parenthesized(parser)?,
            Token::Identifier(ident) => ident.into(),
            Token::SpreadOperator => {
                let next_token = next_token(parser)?;

                match next_token {
                    Token::Identifier(value) => Expr::SpeadId(value),
                    _ => {
                        return Err(AspenError::expected(
                            parser,
                            "an identifier following the '...'".to_owned(),
                        ))
                    }
                }
            }
            Token::Import => {
                expect_token(parser, Token::OpenParen)?;

                let name = match next_jump_multispace(parser)? {
                    Token::String(name) => name,
                    _ => return Err(AspenError::expected(parser, "an import value".to_owned())),
                };

                expect_token(parser, Token::CloseParen)?;

                Expr::Import(name)
            }
            token => parse_value(parser, token)?.into(),
        };

        Ok(expr)
    }

    fn add_func_call_to_most_rhs(&mut self, args: Vec<Box<Expr<'s>>>) {
        let mut expr = self;
        while let Expr::Binary { rhs, .. } = expr {
            expr = rhs;
        }
        *expr = Expr::FuncCall {
            callee: Box::new(expr.clone()),
            args,
        };
    }
    fn add_array_indexing_to_most_rhs(&mut self, indexer: Box<Expr<'s>>) {
        let mut expr = self;
        while let Expr::Binary { rhs, .. } = expr {
            expr = rhs;
        }
        *expr = Expr::ArrayIndexing {
            indexed: Box::new(expr.clone()),
            indexer,
        };
    }
    fn add_obj_indexing_to_most_rhs(&mut self, indexer: Box<Expr<'s>>) {
        let mut expr = self;
        while let Expr::Binary { rhs, .. } = expr {
            expr = rhs;
        }
        *expr = Expr::ObjIndexing {
            indexed: Box::new(expr.clone()),
            indexer,
        };
    }
    fn add_error_propagation_to_most_rhs(&mut self) {
        let mut expr = self;
        while let Expr::Binary { rhs, .. } = expr {
            expr = rhs;
        }
        *expr = Expr::PropagatedFailible(Box::new(expr.clone()));
    }
    fn add_string_concatenation_to_most_rhs(&mut self, right: Box<Expr<'s>>) {
        let mut expr = self;
        while let Expr::Binary { rhs, .. } = expr {
            expr = rhs;
        }
        *expr = Expr::StringConcatenation {
            left: Box::new(expr.clone()),
            right,
        };
    }

    /// Function to call after a '(' is consumed when the expression is expected to be a function call.
    pub fn modify_into_fn_call(
        parser: &mut AspenParser<'s>,
        base_expr: &mut Box<Expr<'s>>,
    ) -> AspenResult<()> {
        let args = Func::parse_call_args(parser)?;
        match base_expr.as_mut() {
            Expr::Id(_)
            | Expr::FuncCall { .. }
            | Expr::ObjIndexing { .. }
            | Expr::ArrayIndexing { .. }
            | Expr::Parenthesized(_)
            | Expr::PropagatedFailible(_) => {
                *base_expr = Box::new(Expr::FuncCall {
                    callee: base_expr.clone(),
                    args,
                });
            }
            Expr::Binary { rhs, .. } => {
                rhs.add_func_call_to_most_rhs(args);
            }
            Expr::Range {
                ref mut end,
                ref mut step,
                ..
            } => {
                match step.is_some() {
                    true => {
                        *step = Some(Box::new(Expr::FuncCall {
                            callee: step.take().unwrap(),
                            args,
                        }));
                    }
                    false => {
                        *end = Box::new(Expr::FuncCall {
                            callee: end.clone(),
                            args,
                        });
                    }
                };
            }
            Expr::Assign {
                ref mut value,
                ref target,
                ref operator,
            } => match value.as_mut() {
                Expr::Binary { rhs, .. } => {
                    rhs.add_func_call_to_most_rhs(args);
                }
                Expr::Id(_)
                | Expr::FuncCall { .. }
                | Expr::ObjIndexing { .. }
                | Expr::ArrayIndexing { .. }
                | Expr::Parenthesized(_)
                | Expr::PropagatedFailible(_) => {
                    *base_expr = Box::new(Expr::Assign {
                        target: target.clone(),
                        operator: operator.to_owned(),
                        value: Box::new(Expr::FuncCall {
                            callee: value.clone(),
                            args,
                        }),
                    });
                }
                Expr::Range {
                    ref mut end,
                    ref mut step,
                    ..
                } => {
                    match step.is_some() {
                        true => {
                            *step = Some(Box::new(Expr::FuncCall {
                                callee: step.take().unwrap(),
                                args,
                            }));
                        }
                        false => {
                            *end = Box::new(Expr::FuncCall {
                                callee: end.clone(),
                                args,
                            });
                        }
                    };
                }
                _ => return Err(AspenError::unknown(parser, "token '(' found".to_owned())),
            },
            _ => return Err(AspenError::unknown(parser, "token '(' found".to_owned())),
        };

        Ok(())
    }

    /// Function to call after a '?' is consumed when the expression is expected to be a failible expression.
    pub fn modify_into_error_propagation(
        _parser: &mut AspenParser<'s>,
        base_expr: &mut Box<Expr<'s>>,
    ) -> AspenResult<()> {
        match base_expr.as_mut() {
            Expr::Binary { rhs, .. } => {
                rhs.add_error_propagation_to_most_rhs();
            }
            Expr::Range {
                ref mut end,
                ref mut step,
                ..
            } => {
                match step.is_some() {
                    true => {
                        *step = Some(Box::new(Expr::PropagatedFailible(step.take().unwrap())));
                    }
                    false => {
                        *end = Box::new(Expr::PropagatedFailible(end.clone()));
                    }
                };
            }
            Expr::Assign {
                ref mut value,
                ref target,
                ref operator,
            } => match value.as_mut() {
                Expr::Binary { rhs, .. } => {
                    rhs.add_error_propagation_to_most_rhs();
                }
                Expr::Range {
                    ref mut end,
                    ref mut step,
                    ..
                } => {
                    match step.is_some() {
                        true => {
                            *step = Some(Box::new(Expr::PropagatedFailible(step.take().unwrap())));
                        }
                        false => {
                            *end = Box::new(Expr::PropagatedFailible(end.clone()));
                        }
                    };
                }
                _ => {
                    *base_expr = Box::new(Expr::Assign {
                        target: target.clone(),
                        operator: operator.to_owned(),
                        value: Box::new(Expr::PropagatedFailible(value.clone())),
                    });
                }
            },
            _ => {
                *base_expr = Box::new(Expr::PropagatedFailible(base_expr.clone()));
            }
        };

        Ok(())
    }

    /// Function to call after a '[' is consumed when the expression is expected to be an array indexing.
    pub fn modify_into_array_indexing(
        parser: &mut AspenParser<'s>,
        base_expr: &mut Box<Expr<'s>>,
    ) -> AspenResult<()> {
        let (expr, _) = Expr::parse_until(parser, &[Token::CloseBracket])?;
        match base_expr.as_mut() {
            Expr::Id(_)
            | Expr::FuncCall { .. }
            | Expr::ObjIndexing { .. }
            | Expr::ArrayIndexing { .. }
            | Expr::Parenthesized(_)
            | Expr::Value(Value::Str(_))
            | Expr::PropagatedFailible(_) => {
                *base_expr = Box::new(Expr::ArrayIndexing {
                    indexed: base_expr.clone(),
                    indexer: expr,
                });
            }
            Expr::Range {
                ref mut end,
                ref mut step,
                ..
            } => {
                match step.is_some() {
                    true => {
                        *step = Some(Box::new(Expr::ArrayIndexing {
                            indexed: step.take().unwrap(),
                            indexer: expr,
                        }));
                    }
                    false => {
                        *end = Box::new(Expr::ArrayIndexing {
                            indexed: end.clone(),
                            indexer: expr,
                        });
                    }
                };
            }
            Expr::Binary { rhs, .. } => rhs.add_array_indexing_to_most_rhs(expr),
            Expr::Assign {
                value,
                target,
                operator,
            } => match value.as_mut() {
                Expr::Binary { rhs, .. } => rhs.add_array_indexing_to_most_rhs(expr),
                Expr::Id(_)
                | Expr::FuncCall { .. }
                | Expr::ObjIndexing { .. }
                | Expr::ArrayIndexing { .. }
                | Expr::Parenthesized(_)
                | Expr::Value(Value::Str(_))
                | Expr::PropagatedFailible(_) => {
                    *base_expr = Box::new(Expr::Assign {
                        target: target.clone(),
                        operator: operator.clone(),
                        value: Box::new(Expr::ArrayIndexing {
                            indexed: value.clone(),
                            indexer: expr,
                        }),
                    });
                }
                Expr::Range {
                    ref mut end,
                    ref mut step,
                    ..
                } => {
                    match step.is_some() {
                        true => {
                            *step = Some(Box::new(Expr::ArrayIndexing {
                                indexed: step.take().unwrap(),
                                indexer: expr,
                            }));
                        }
                        false => {
                            *end = Box::new(Expr::ArrayIndexing {
                                indexed: end.clone(),
                                indexer: expr,
                            });
                        }
                    };
                }
                _ => return Err(AspenError::unknown(parser, "token found: '['".to_owned())),
            },
            _ => return Err(AspenError::unknown(parser, "token found: '['".to_owned())),
        };

        Ok(())
    }

    /// Function to call after a '.' is consumed when the expression is expected to be an obj indexing.
    pub fn modify_into_obj_indexing(
        parser: &mut AspenParser<'s>,
        base_expr: &mut Box<Expr<'s>>,
    ) -> AspenResult<()> {
        let e = Expr::parse(parser)?;

        match &e {
            Expr::Id(_) => (),
            _ => {
                return Err(AspenError::unknown(
                    parser,
                    format!(
                        "token '{:?}', an object can only be accessed with an identifier",
                        e
                    ),
                ));
            }
        }

        let expr = Box::new(e);
        match base_expr.as_mut() {
            Expr::Value(_)
            | Expr::Id(_)
            | Expr::FuncCall { .. }
            | Expr::ObjIndexing { .. }
            | Expr::ArrayIndexing { .. }
            | Expr::Parenthesized(_)
            | Expr::PropagatedFailible(_) => {
                *base_expr = Box::new(Expr::ObjIndexing {
                    indexed: base_expr.clone(),
                    indexer: expr,
                });
            }
            Expr::Range {
                ref mut end,
                ref mut step,
                ..
            } => {
                match step.is_some() {
                    true => {
                        *step = Some(Box::new(Expr::ObjIndexing {
                            indexed: step.take().unwrap(),
                            indexer: expr,
                        }));
                    }
                    false => {
                        *end = Box::new(Expr::ObjIndexing {
                            indexed: end.clone(),
                            indexer: expr,
                        });
                    }
                };
            }
            Expr::Binary { rhs, .. } => rhs.add_obj_indexing_to_most_rhs(expr),
            Expr::Assign {
                value,
                target,
                operator,
            } => match value.as_mut() {
                Expr::Binary { rhs, .. } => rhs.add_obj_indexing_to_most_rhs(expr),
                Expr::Value(_)
                | Expr::Id(_)
                | Expr::FuncCall { .. }
                | Expr::ObjIndexing { .. }
                | Expr::ArrayIndexing { .. }
                | Expr::Parenthesized(_)
                | Expr::PropagatedFailible(_) => {
                    *base_expr = Box::new(Expr::Assign {
                        target: target.clone(),
                        operator: operator.clone(),
                        value: Box::new(Expr::ObjIndexing {
                            indexed: value.clone(),
                            indexer: expr,
                        }),
                    });
                }
                Expr::Range {
                    ref mut end,
                    ref mut step,
                    ..
                } => {
                    match step.is_some() {
                        true => {
                            *step = Some(Box::new(Expr::ObjIndexing {
                                indexed: step.take().unwrap(),
                                indexer: expr,
                            }));
                        }
                        false => {
                            *end = Box::new(Expr::ObjIndexing {
                                indexed: end.clone(),
                                indexer: expr,
                            });
                        }
                    };
                }
                _ => return Err(AspenError::unknown(parser, "token found: '.'".to_owned())),
            },
            _ => return Err(AspenError::unknown(parser, "token found: '.'".to_owned())),
        };

        Ok(())
    }

    /// Function to call after a '..' is consumed when the expression is expected to be a string concatenation.
    pub fn modify_into_string_concatenation(
        parser: &mut AspenParser<'s>,
        base_expr: &mut Box<Expr<'s>>,
    ) -> AspenResult<()> {
        let e = Expr::parse(parser)?;

        let expr = Box::new(e);
        match base_expr.as_mut() {
            Expr::Value(Value::Str(_))
            | Expr::Id(_)
            | Expr::FuncCall { .. }
            | Expr::ObjIndexing { .. }
            | Expr::ArrayIndexing { .. }
            | Expr::Parenthesized(_)
            | Expr::PropagatedFailible(_) => {
                *base_expr = Box::new(Expr::StringConcatenation {
                    left: base_expr.clone(),
                    right: expr,
                });
            }
            Expr::StringConcatenation { ref mut right, .. } => {
                *right = Box::new(Expr::StringConcatenation {
                    left: right.clone(),
                    right: expr,
                })
            }
            Expr::Binary { rhs, .. } => rhs.add_string_concatenation_to_most_rhs(expr),
            Expr::Assign {
                value,
                target,
                operator,
            } => match value.as_mut() {
                Expr::Binary { rhs, .. } => rhs.add_string_concatenation_to_most_rhs(expr),
                Expr::Value(Value::Str(_))
                | Expr::Id(_)
                | Expr::FuncCall { .. }
                | Expr::ObjIndexing { .. }
                | Expr::ArrayIndexing { .. }
                | Expr::Parenthesized(_)
                | Expr::PropagatedFailible(_) => {
                    *base_expr = Box::new(Expr::Assign {
                        target: target.clone(),
                        operator: operator.clone(),
                        value: Box::new(Expr::StringConcatenation {
                            left: value.clone(),
                            right: expr,
                        }),
                    });
                }
                Expr::StringConcatenation { ref mut right, .. } => {
                    *right = Box::new(Expr::StringConcatenation {
                        left: right.clone(),
                        right: expr,
                    })
                }
                _ => return Err(AspenError::unknown(parser, "token found: '..'".to_owned())),
            },
            _ => {
                return Err(AspenError::unknown(parser, "token found: '..'".to_owned()));
            }
        };

        Ok(())
    }

    /// Function to call after a ':' is consumed when the expression is expected to be a range.
    pub fn modify_into_range(
        parser: &mut AspenParser<'s>,
        base_expr: &mut Box<Expr<'s>>,
    ) -> AspenResult<()> {
        let second_expr = Expr::parse(parser)?;

        match base_expr.as_mut() {
            Expr::Range { start, end, step } => {
                if step.is_some() {
                    return Err(AspenError::unknown(
                        parser,
                        "token ':' found, a Range has three parts: start:end:step".to_owned(),
                    ));
                }

                *base_expr = Box::new(Expr::Range {
                    start: start.clone(),
                    end: end.clone(),
                    step: Some(Box::new(second_expr)),
                });
            }
            Expr::Id(_)
            | Expr::Binary { .. }
            | Expr::FuncCall { .. }
            | Expr::Value(_)
            | Expr::Parenthesized(_)
            | Expr::ObjIndexing { .. }
            | Expr::ArrayIndexing { .. }
            | Expr::PropagatedFailible(_) => {
                *base_expr = Box::new(Expr::Range {
                    start: base_expr.clone(),
                    end: Box::new(second_expr),
                    step: None,
                });
            }
            Expr::Assign {
                ref mut value,
                operator,
                target,
            } => match value.as_mut() {
                Expr::Range { start, end, step } => {
                    if step.is_some() {
                        return Err(AspenError::unknown(
                            parser,
                            "token ':' found, a Range has three parts: start:end:step".to_owned(),
                        ));
                    }

                    *base_expr = Box::new(Expr::Assign {
                        target: target.clone(),
                        operator: operator.clone(),
                        value: Expr::Range {
                            start: start.clone(),
                            end: end.clone(),
                            step: Some(Box::new(second_expr)),
                        }
                        .into(),
                    });
                }
                Expr::Id(_)
                | Expr::Binary { .. }
                | Expr::FuncCall { .. }
                | Expr::Value(_)
                | Expr::Parenthesized(_)
                | Expr::ObjIndexing { .. }
                | Expr::ArrayIndexing { .. }
                | Expr::PropagatedFailible(_) => {
                    *base_expr = Box::new(Expr::Assign {
                        target: target.clone(),
                        operator: operator.clone(),
                        value: Expr::Range {
                            start: value.clone(),
                            end: Box::new(second_expr),
                            step: None,
                        }
                        .into(),
                    });
                }
                _ => return Err(AspenError::unknown(parser, "token ':' found".to_owned())),
            },
            _ => return Err(AspenError::unknown(parser, "token ':' found".to_owned())),
        };

        Ok(())
    }

    /// Function to call after a [`BinaryOperator`] is consumed when the expression is expected to be a binary operation.
    pub fn modify_into_binary_op(
        parser: &mut AspenParser,
        base_expr: &mut Box<Expr<'s>>,
        right_expr: Expr<'s>,
        bop: BinaryOperator,
    ) -> AspenResult<()> {
        match base_expr.as_mut() {
            Expr::Binary { lhs, operator, rhs } => {
                let result = operator.get_precedence().cmp(&bop.get_precedence());
                match result {
                    Ordering::Greater => {
                        *base_expr = Box::new(Expr::Binary {
                            lhs: base_expr.clone(),
                            operator: bop,
                            rhs: Box::new(right_expr),
                        });
                    }
                    Ordering::Equal | Ordering::Less => {
                        *base_expr = Expr::Binary {
                            lhs: lhs.clone(),
                            operator: operator.clone(),
                            rhs: Box::new(Expr::Binary {
                                lhs: rhs.clone(),
                                operator: bop,
                                rhs: Box::new(right_expr),
                            }),
                        }
                        .into();
                    }
                }
            }
            Expr::Assign {
                target,
                operator: aop,
                value,
            } => match value.as_mut() {
                Expr::Binary { rhs, operator, lhs } => {
                    let result = operator.get_precedence().cmp(&bop.get_precedence());
                    match result {
                        Ordering::Greater => {
                            *base_expr = Expr::Assign {
                                target: target.clone(),
                                operator: aop.clone(),
                                value: Box::new(Expr::Binary {
                                    lhs: base_expr.clone(),
                                    operator: bop,
                                    rhs: Box::new(right_expr),
                                }),
                            }
                            .into();
                        }
                        Ordering::Equal | Ordering::Less => {
                            *base_expr = Expr::Assign {
                                target: target.clone(),
                                operator: aop.clone(),
                                value: Box::new(
                                    Expr::Binary {
                                        lhs: lhs.clone(),
                                        operator: operator.clone(),
                                        rhs: Box::new(Expr::Binary {
                                            lhs: rhs.clone(),
                                            operator: bop,
                                            rhs: Box::new(right_expr),
                                        }),
                                    }
                                    .into(),
                                ),
                            }
                            .into();
                        }
                    }
                }
                _ => {
                    *base_expr = Expr::Assign {
                        target: target.clone(),
                        operator: aop.clone(),
                        value: Box::new(Expr::Binary {
                            lhs: value.clone(),
                            operator: bop,
                            rhs: Box::new(right_expr),
                        }),
                    }
                    .into();
                }
            },
            _ => {
                *base_expr = Expr::Binary {
                    lhs: base_expr.clone(),
                    operator: bop,
                    rhs: Box::new(right_expr),
                }
                .into();
            }
        };

        Ok(())
    }

    /// Parses a parenthesized expr.
    ///
    /// **NOTE: We assume '(' was already consumed! And parses the ending ')'**
    pub fn parse_parenthesized(parser: &mut AspenParser<'s>) -> AspenResult<Expr<'s>> {
        Ok(Expr::Parenthesized(
            Self::parse_until(parser, &[Token::CloseParen])?.0,
        ))
    }

    pub fn parse_until(
        parser: &mut AspenParser<'s>,
        stop_tokens: &[Token<'s>],
    ) -> AspenResult<(Box<Expr<'s>>, Token<'s>)> {
        let mut base_expr = Box::new(Expr::parse(parser)?);
        let mut bop: Option<BinaryOperator> = None; // bop for binary operator

        loop {
            match next_jump_multispace(parser)? {
                Token::LineComment(val) | Token::DocComment(val) | Token::MultiLineComment(val) => {
                    let start = parser.lexer.span().start;
                    let end = parser.lexer.span().end;
                    parser.add_comment(Comment::new(val, start, end))
                }
                token if bop.is_none() => match token {
                    Token::BinaryOperator(op) => {
                        bop = Some(op);
                    }
                    Token::OpenParen => Expr::modify_into_fn_call(parser, &mut base_expr)?,
                    Token::OpenBracket => Expr::modify_into_array_indexing(parser, &mut base_expr)?,
                    Token::Dot => Expr::modify_into_obj_indexing(parser, &mut base_expr)?,
                    Token::PropagationOperator => {
                        Expr::modify_into_error_propagation(parser, &mut base_expr)?
                    }
                    Token::StringSeparator => {
                        Expr::modify_into_string_concatenation(parser, &mut base_expr)?
                    }
                    token if stop_tokens.contains(&token) => return Ok((base_expr, token)),
                    _ => {
                        return Err(AspenError::unknown(
                            parser,
                            format!("token '{:?}' found", token),
                        ))
                    }
                },
                token if bop.is_some() => {
                    let right_expr = Expr::parse_with_token(parser, token)?;
                    Expr::modify_into_binary_op(
                        parser,
                        &mut base_expr,
                        right_expr,
                        bop.take().unwrap(),
                    )?;
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
    // we could simplify this function using Expr::parser_until, maybe in the future ?
    let mut arr = Vec::new();
    let mut awaits_comma = false;

    loop {
        match next_jump_multispace(parser)? {
            Token::Comma if awaits_comma => awaits_comma = false,
            Token::CloseBracket => return Ok(arr),
            Token::LineComment(val) | Token::DocComment(val) | Token::MultiLineComment(val) => {
                let start = parser.lexer.span().start;
                let end = parser.lexer.span().end;
                parser.add_comment(Comment::new(val, start, end))
            }
            token if !awaits_comma => {
                let expr = Expr::parse_with_token(parser, token)?;
                arr.push(Box::new(expr));
                awaits_comma = true;
            }
            mut token if awaits_comma => {
                let base_expr = arr.last_mut().unwrap();
                let mut bop: Option<BinaryOperator> = None;

                loop {
                    match token {
                        token if bop.is_none() => match token {
                            Token::BinaryOperator(op) => {
                                bop = Some(op);
                            }
                            Token::OpenParen => Expr::modify_into_fn_call(parser, base_expr)?,
                            Token::Comma => {
                                awaits_comma = false;
                                break;
                            }
                            Token::CloseBracket => return Ok(arr),
                            _ => {
                                return Err(AspenError::expected(
                                    parser,
                                    "a close bracket ']'".to_owned(),
                                ))
                            }
                        },
                        token if bop.is_some() => {
                            let right_expr = Expr::parse_with_token(parser, token)?;
                            Expr::modify_into_binary_op(
                                parser,
                                base_expr,
                                right_expr,
                                bop.take().unwrap(),
                            )?;
                        }
                        _ => unreachable!(),
                    }

                    token = next_jump_multispace(parser)?;
                }
            }

            _ => unreachable!("All cases are covered up there!"),
        };
    }
}

/// Parses an object.
///
/// **NOTE: We assume "{" was already consumed!**
fn parse_obj<'s>(parser: &mut AspenParser<'s>) -> AspenResult<HashMap<&'s str, Box<Expr<'s>>>> {
    // we could simplify this function using Expr::parser_until, maybe in the future ?
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
            Token::ObjectKey(k) if key.is_some() => {
                hash.insert(key.take().unwrap(), value.clone().take().unwrap());
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
            Token::OpenBrace if key.is_some() => {
                let object = parse_obj(parser)?;
                value = Some(Box::new(object.into()));
            }
            Token::OpenParen if value.is_some() => {
                let mut val = value.take().unwrap();
                Expr::modify_into_fn_call(parser, &mut val)?;
                value = Some(val);
            }
            Token::PropagationOperator if value.is_some() => {
                let mut val = value.take().unwrap();
                Expr::modify_into_error_propagation(parser, &mut val)?;
                value = Some(val);
            }
            Token::OpenBracket if key.is_some() => {
                let sub_array = parse_array(parser)?;
                value = Some(Box::new(sub_array.into()));
            }
            Token::Identifier(ident) if key.is_some() => {
                value = Some(Box::new(ident.into()));
            }
            Token::Identifier(ident) => {
                key = Some(ident);
                value = Some(Box::new(ident.into()));
            }
            Token::BinaryOperator(op) if key.is_some() => {
                let expr = Expr::parse(parser)?;
                let mut val = value.take().unwrap();
                Expr::modify_into_binary_op(parser, &mut val, expr, op)?;
                value = Some(val);
            }
            Token::SpreadOperator if key.is_some() => return Err(AspenError::expected(
                parser,
                "an object or an array: either do {..<spread_variable>} or [...<spread_variable>]"
                    .to_owned(),
            )),
            Token::SpreadOperator => {
                let next_token = next_token(parser)?;

                match next_token {
                    Token::Identifier(ident) => {
                        key = Some(ident);
                        value = Some(Box::new(Expr::SpeadId(ident)));
                    }
                    _ => {
                        return Err(AspenError::expected(
                            parser,
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
                value = Some(Box::new(Expr::parse_with_token(parser, token)?));
            }
            _ => return Err(AspenError::expected(parser, "a valid <expr>".to_owned())),
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

impl<'a> From<Vec<Box<Expr<'a>>>> for Expr<'a> {
    fn from(val: Vec<Box<Expr<'a>>>) -> Self {
        Expr::Array(val)
    }
}

impl<'a> From<HashMap<&'a str, Box<Expr<'a>>>> for Expr<'a> {
    fn from(val: HashMap<&'a str, Box<Expr<'a>>>) -> Self {
        Expr::Object(val)
    }
}

impl<'a> From<&'a str> for Expr<'a> {
    fn from(val: &'a str) -> Self {
        Expr::Id(val)
    }
}

use std::fmt;

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Value(value) => write!(f, "{:?}", value),
            Expr::Import(s) => write!(f, "$import(\"{}\")", s),
            Expr::Array(exprs) => {
                write!(f, "[")?;
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", expr)?;
                }
                write!(f, "]")
            }
            Expr::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, expr)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}! {}", key, expr)?;
                }
                write!(f, "}}")
            }
            Expr::Id(s) => write!(f, "{}", s),
            Expr::SpeadId(s) => write!(f, "...{}", s),
            Expr::Parenthesized(expr) => write!(f, "({})", expr),
            Expr::Assign {
                target,
                operator,
                value,
            } => {
                write!(f, "{} {} {}", target, operator, value)
            }
            Expr::Binary { lhs, operator, rhs } => {
                write!(f, "{} {} {}", lhs, operator, rhs)
            }
            Expr::FuncCall { callee, args } => {
                write!(f, "{}(", callee)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::Range { start, end, step } => {
                write!(f, "{}..{}", start, end)?;
                if let Some(step) = step {
                    write!(f, "..{}", step)?;
                }
                Ok(())
            }
            Expr::ArrayIndexing { indexed, indexer } => {
                write!(f, "{}[{}]", indexed, indexer)
            }
            Expr::ObjIndexing { indexed, indexer } => {
                write!(f, "{}.{}", indexed, indexer)
            }
            Expr::StringConcatenation { left, right } => {
                write!(f, "{}..{}", left, right)
            }
            Expr::PropagatedFailible(expr) => {
                write!(f, "{}?", expr)
            }
        }
    }
}
