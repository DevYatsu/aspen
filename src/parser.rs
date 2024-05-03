use self::conditional::If;
use self::error::AspenError;
use self::for_loop::For;
use self::func::Func;
use self::operator::{AssignOperator, BinaryOperator};
use self::return_stmt::Return;
use self::utils::Block;
use self::while_loop::While;
use self::{comment::Comment, error::AspenResult, value::Value, var::Var};
use crate::lexer::{AspenLexer, Token};
use hashbrown::HashMap;
use logos::Lexer;

pub mod comment;
pub mod conditional;
pub mod error;
mod expr;
pub mod for_loop;
pub mod func;
mod macros;
pub mod operator;
pub mod return_stmt;
pub mod utils;
pub mod value;
pub mod var;
pub mod while_loop;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Var(Var<'a>),
    Func(Func<'a>),

    // change to Expr{value: Box<Expr<'a>>, is_returned: bool} to handle implicit return in the future, when followed by ';' then is not returned
    Expr(Box<Expr<'a>>),

    For(For<'a>),
    While(While<'a>),
    Return(Return<'a>),
    If(If<'a>),
}

pub type Container<T> = Vec<Box<T>>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents an Aspen Expression!
pub enum Expr<'a> {
    Value(Value<'a>),
    Import(&'a str),

    Array(Container<Expr<'a>>),
    Object(HashMap<&'a str, Box<Expr<'a>>>),

    Id(&'a str),
    SpeadId(&'a str),

    Parenthesized(Box<Expr<'a>>),
    PropagatedFailible(Box<Expr<'a>>),

    Assign {
        target: Box<Expr<'a>>,
        operator: AssignOperator,
        value: Box<Expr<'a>>,
    },
    Binary {
        lhs: Box<Expr<'a>>,
        operator: BinaryOperator,
        rhs: Box<Expr<'a>>,
    },

    FuncCall {
        callee: Box<Expr<'a>>,
        args: Vec<Box<Expr<'a>>>,
    },

    Range {
        start: Box<Expr<'a>>,
        end: Box<Expr<'a>>,
        step: Option<Box<Expr<'a>>>,
    },

    ArrayIndexing {
        indexed: Box<Expr<'a>>,
        indexer: Box<Expr<'a>>,
    },

    ObjIndexing {
        indexed: Box<Expr<'a>>,
        indexer: Box<Expr<'a>>,
    },

    StringConcatenation {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>,
    },
}

#[derive(Debug, Clone)]
pub struct AspenParser<'s> {
    pub lexer: AspenLexer<'s>,
    body: Block<'s>,
    comments: Container<Comment<'s>>,
}

pub fn parse_aspen(parser: &mut AspenParser<'_>) -> AspenResult<()> {
    let result = parse_block(parser, None)?;

    parser.body = result;

    Ok(())
}

/// Parses code into a block of statements.
///
/// If stop_on is set, the parsing will stop when the given token is encountered, can be used e.g `}` for a function block ending.
pub fn parse_block<'s>(
    parser: &mut AspenParser<'s>,
    stop_on: Option<Token<'s>>,
) -> AspenResult<Block<'s>> {
    let mut statements = vec![];
    let mut expect_stmt_end = false;
    let mut semi_colon_found = false;

    while let Some(result_token) = parser.lexer.next() {
        let token = result_token.map_err(|e| AspenError::from_lexing_error(parser, e))?;

        match token {
            Token::Return => {
                semi_colon_found = false;
                let stmt = Return::parse(parser)?;
                statements.push(Box::new(stmt));

                expect_stmt_end = true;
                continue;
            }
            Token::Let => {
                semi_colon_found = false;
                let stmt = Var::parse(parser)?;
                statements.push(Box::new(stmt));

                expect_stmt_end = true;
                continue;
            }
            Token::Comma if expect_stmt_end => {
                if semi_colon_found {
                    return Err(AspenError::unknown(parser, "token ',' found".to_owned()));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Var(_) => {
                            let stmt = Var::parse_after_comma(parser)?;
                            statements.push(Box::new(stmt));
                        }
                        _ => {
                            return Err(error::AspenError::unknown(
                                parser,
                                "token ',' found".to_owned(),
                            ))
                        }
                    };
                } else {
                    return Err(error::AspenError::unknown(
                        parser,
                        "token ',' found".to_owned(),
                    ));
                };
                continue;
            }

            Token::For => {
                semi_colon_found = false;
                let stmt = For::parse(parser)?;
                statements.push(Box::new(stmt));
                continue;
            }
            Token::While => {
                semi_colon_found = false;
                let stmt = While::parse(parser)?;
                statements.push(Box::new(stmt));
                continue;
            }
            Token::If => {
                semi_colon_found = false;
                let stmt = If::parse(parser)?;
                statements.push(Box::new(stmt));
                continue;
            }
            Token::Other => {
                if semi_colon_found {
                    return Err(AspenError::unknown(
                        parser,
                        format!("token '{}' found", parser.lexer.slice()),
                    ));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::If(ref mut if_stmt) => {
                            let value = If::parse_other(parser)?;
                            if_stmt.add_other_at_if_end(parser, value)?;
                            continue;
                        }
                        _ => {
                            return Err(AspenError::unknown(
                                parser,
                                format!("token '{}' found", parser.lexer.slice()),
                            ))
                        }
                    };
                }
            }
            Token::Else => {
                if semi_colon_found {
                    return Err(AspenError::unknown(parser, "token 'else' found".to_owned()));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::If(ref mut if_stmt) => {
                            let value = If::parse_else(parser)?;
                            if_stmt.add_other_at_if_end(parser, value)?;
                            continue;
                        }
                        _ => {
                            return Err(AspenError::unknown(
                                parser,
                                format!("token '{}' found", parser.lexer.slice()),
                            ))
                        }
                    };
                }
            }
            Token::LineComment(value)
            | Token::DocComment(value)
            | Token::MultiLineComment(value) => {
                let start = parser.lexer.span().start;
                let end = parser.lexer.span().end;

                parser.add_comment(Comment::new(value, start, end));
                continue;
            }
            Token::Func(name) => {
                semi_colon_found = false;
                let stmt = Func::parse(parser, name)?;
                statements.push(Box::new(stmt));
                continue;
            }
            _ if stop_on.is_some() && &token == stop_on.as_ref().unwrap() => {
                return Ok(Block::new(statements));
            }
            Token::Nil
            | Token::Bool(_)
            | Token::Float(_)
            | Token::Int(_)
            | Token::OpenBrace
            | Token::SpreadOperator
            | Token::String(_)
            | Token::Identifier(_) => {
                if let Ok(ex) = Expr::parse_with_token(parser, token) {
                    statements.push(Box::new(ex.into()))
                }
            }
            Token::AssignOperator(aop) => {
                if semi_colon_found {
                    return Err(AspenError::unknown(
                        parser,
                        format!("token '{}' found", aop),
                    ));
                }

                if let Some(stmt) = statements.last_mut() {
                    if let Statement::Expr(base_expr) = stmt.as_mut() {
                        let expr = Expr::parse(parser)?;
                        **stmt = Expr::Assign {
                            target: base_expr.clone(),
                            operator: aop,
                            value: Box::new(expr),
                        }
                        .into();
                    } else {
                        return Err(error::AspenError::unknown(
                            parser,
                            format!("token '{}' found", aop),
                        ));
                    }
                } else {
                    return Err(error::AspenError::unknown(
                        parser,
                        format!("token '{}' found", aop),
                    ));
                };
            }
            Token::BinaryOperator(bop) => {
                if semi_colon_found {
                    return Err(AspenError::unknown(
                        parser,
                        format!("token '{}' found", bop),
                    ));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr)
                        | Statement::Var(Var {
                            value: base_expr, ..
                        }) => {
                            let expr = Expr::parse(parser)?;
                            Expr::modify_into_binary_op(parser, base_expr, expr, bop)?;
                        }
                        Statement::Return(Return(returned_expr)) => {
                            let expr = Expr::parse(parser)?;
                            Expr::modify_into_binary_op(parser, returned_expr, expr, bop)?;
                        }
                        _ => {
                            return Err(error::AspenError::unknown(
                                parser,
                                format!("token '{}' found", bop),
                            ))
                        }
                    };
                } else {
                    return Err(error::AspenError::unknown(
                        parser,
                        format!("token '{}' found", bop),
                    ));
                };
            }
            Token::OpenParen => {
                if semi_colon_found {
                    return Err(AspenError::unknown(parser, "token '(' found".to_owned()));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr) => {
                            Expr::modify_into_fn_call(parser, base_expr)?;
                            continue;
                        }
                        Statement::Var(Var { value, .. }) => {
                            Expr::modify_into_fn_call(parser, value)?;
                            continue;
                        }
                        Statement::Return(Return(returned_expr)) => {
                            Expr::modify_into_fn_call(parser, returned_expr)?;
                            continue;
                        }
                        _ => (),
                    };
                }

                let expr = Expr::parse_parenthesized(parser)?;
                statements.push(Box::new(Statement::Expr(Box::new(expr))))
            }
            Token::PropagationOperator => {
                if semi_colon_found {
                    return Err(AspenError::unknown(parser, "token '?' found".to_owned()));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr) => {
                            Expr::modify_into_fn_call(parser, base_expr)?;
                            continue;
                        }
                        Statement::Var(Var { value, .. }) => {
                            Expr::modify_into_fn_call(parser, value)?;
                            continue;
                        }
                        Statement::Return(Return(returned_expr)) => {
                            Expr::modify_into_fn_call(parser, returned_expr)?;
                            continue;
                        }
                        _ => (),
                    };
                }

                return Err(AspenError::unknown(parser, "token '?' found".to_owned()));
            }
            Token::Range => {
                if semi_colon_found {
                    return Err(AspenError::unknown(parser, "token ':' found".to_owned()));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr) => {
                            Expr::modify_into_range(parser, base_expr)?;
                            continue;
                        }
                        Statement::Var(var) => {
                            let Var { value, .. } = var;
                            Expr::modify_into_range(parser, value)?;
                            continue;
                        }
                        _ => (),
                    };
                }

                return Err(AspenError::unknown(parser, "token ':' found".to_owned()));
            }
            Token::Dot => {
                if semi_colon_found {
                    return Err(AspenError::unknown(parser, "token '.' found".to_owned()));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr) => {
                            Expr::modify_into_obj_indexing(parser, base_expr)?;
                        }
                        Statement::Var(Var { value, .. }) => {
                            Expr::modify_into_obj_indexing(parser, value)?;
                        }
                        Statement::Return(Return(returned_expr)) => {
                            Expr::modify_into_obj_indexing(parser, returned_expr)?;
                        }
                        _ => return Err(AspenError::unknown(parser, "token '[' found".to_owned())),
                    };
                }
            }
            Token::StringSeparator => {
                if semi_colon_found {
                    return Err(AspenError::unknown(parser, "token '..' found".to_owned()));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr) => {
                            Expr::modify_into_string_concatenation(parser, base_expr)?;
                        }
                        Statement::Var(Var { value, .. }) => {
                            Expr::modify_into_string_concatenation(parser, value)?;
                        }
                        Statement::Return(Return(returned_expr)) => {
                            Expr::modify_into_string_concatenation(parser, returned_expr)?;
                        }
                        _ => {
                            return Err(AspenError::unknown(parser, "token '..' found".to_owned()))
                        }
                    };
                }
            }
            Token::OpenBracket => {
                if semi_colon_found {
                    return Err(AspenError::unknown(
                        parser,
                        "token '[' found, cannot write arrays expressions in global context"
                            .to_owned(),
                    ));
                }

                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr) => {
                            Expr::modify_into_array_indexing(parser, base_expr)?;
                        }
                        Statement::Var(Var { value, .. }) => {
                            Expr::modify_into_array_indexing(parser, value)?;
                        }
                        Statement::Return(Return(returned_expr)) => {
                            Expr::modify_into_array_indexing(parser, returned_expr)?;
                        }
                        _ => return Err(AspenError::unknown(parser, "token '[' found".to_owned())),
                    };
                }
            }
            Token::Newline => expect_stmt_end = false,
            Token::SemiColon => {
                expect_stmt_end = false;
                semi_colon_found = true;
            }
            Token::Spaces => (),
            _ => {
                if expect_stmt_end {
                    return Err(AspenError::expected_newline(parser));
                } else {
                    return Err(AspenError::unknown(
                        parser,
                        format!("token '{}' found", parser.lexer.slice()),
                    ));
                }
            }
        }
    }

    if stop_on.is_some() {
        return Err(error::AspenError::expected(
            parser,
            format!("token '{:?}'", stop_on),
        ));
    }

    Ok(Block::new(statements))
}

impl<'a> AspenParser<'a> {
    fn new(lexer: Lexer<'a, Token<'a>>) -> Self {
        Self {
            lexer,
            body: Block::default(),
            comments: vec![],
        }
    }
    pub fn add_comment(&mut self, comment: Comment<'a>) {
        self.comments.push(Box::new(comment))
    }
    pub fn statements(&mut self) -> Container<Statement<'a>> {
        self.body.statements()
    }
    pub fn comments(&self) -> Container<Comment<'a>> {
        self.comments.to_owned()
    }
}

impl<'a> From<Lexer<'a, Token<'a>>> for AspenParser<'a> {
    fn from(value: Lexer<'a, Token<'a>>) -> Self {
        Self::new(value)
    }
}

impl<'a> From<Expr<'a>> for Statement<'a> {
    fn from(value: Expr<'a>) -> Self {
        Statement::Expr(Box::new(value))
    }
}
