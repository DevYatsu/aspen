use self::error::AspenError;
use self::for_loop::For;
use self::func::Func;
use self::operator::{AssignOperator, BinaryOperator};
use self::utils::Block;
use self::while_loop::While;
use self::{comment::Comment, error::AspenResult, import::Import, value::Value, var::Var};
use crate::{
    lexer::{AspenLexer, Token},
    parser::utils::expect_newline,
};
use hashbrown::HashMap;
use logos::Lexer;

pub mod comment;
pub mod error;
mod expr;
pub mod for_loop;
pub mod func;
pub mod import;
mod macros;
pub mod operator;
pub mod utils;
pub mod value;
pub mod var;
pub mod while_loop;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Var(Var<'a>),
    Import(Import<'a>),
    Func(Func<'a>),
    Expr(Box<Expr<'a>>),
    For(For<'a>),
    While(While<'a>),
}

pub type Container<T> = Vec<Box<T>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Value(Value<'a>),

    Array(Container<Expr<'a>>),
    Object(HashMap<&'a str, Expr<'a>>),

    Id(&'a str),
    SpeadId(&'a str),

    Parenthesized(Box<Expr<'a>>),

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
}

#[derive(Debug, Clone)]
pub struct AspenParser<'s> {
    pub lexer: AspenLexer<'s>,
    body: Block<'s>,
    comments: Container<Comment<'s>>,
}

pub fn parse_aspen<'s>(parser: &mut AspenParser<'s>) -> AspenResult<()> {
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

    while let Some(result_token) = parser.lexer.next() {
        let token = result_token?;

        match token {
            Token::Import => {
                let stmt = Import::parse(parser)?;
                statements.push(Box::new(stmt));
                expect_newline(parser)?;
            }

            Token::Let => {
                let stmt = Var::parse(parser)?;
                statements.push(Box::new(stmt));

                Var::parse_several_vars_or_complex_expr_or_newline(parser, &mut statements)?;
            }
            Token::Comma => {
                if let Some(stmt) = statements.last() {
                    if let Statement::Var(_) = **stmt {
                        let stmt = Var::parse_after_comma(parser)?;
                        statements.push(Box::new(stmt));
                    } else {
                        return Err(error::AspenError::Unknown("token ',' found".to_owned()));
                    }
                } else {
                    return Err(error::AspenError::Unknown("token ',' found".to_owned()));
                };
            }

            Token::For => {
                let stmt = For::parse(parser)?;
                statements.push(Box::new(stmt));
            }
            Token::While => {
                let stmt = While::parse(parser)?;
                statements.push(Box::new(stmt));
            }
            Token::LineComment(value)
            | Token::DocComment(value)
            | Token::MultiLineComment(value) => {
                let start = parser.lexer.span().start;
                let end = parser.lexer.span().end;

                parser.add_comment(Comment::new(value, start, end))
            }
            Token::Func(name) => {
                let stmt = Func::parse(parser, name)?;
                statements.push(Box::new(stmt));
            }
            _ if stop_on.is_some() && &token == stop_on.as_ref().unwrap() => {
                return Ok(Block::new(statements));
            }

            Token::Nil
            | Token::Bool(_)
            | Token::Float(_)
            | Token::Int(_)
            | Token::OpenBrace
            | Token::OpenBracket
            | Token::SpreadOperator
            | Token::String(_)
            | Token::Identifier(_) => {
                if let Ok(ex) = Expr::parse_with_token(parser, token) {
                    statements.push(Box::new(ex.into()))
                }
            }
            Token::AssignOperator(aop) => {
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
                        return Err(error::AspenError::Unknown(format!("token '{}' found", aop)));
                    }
                } else {
                    return Err(error::AspenError::Unknown(format!("token '{}' found", aop)));
                };
            }
            Token::BinaryOperator(bop) => {
                if let Some(stmt) = statements.last_mut() {
                    if let Statement::Expr(base_expr) = stmt.as_mut() {
                        let expr = Expr::parse(parser)?;
                        Expr::modify_into_binary_op(base_expr, expr, bop)?;
                    } else {
                        return Err(error::AspenError::Unknown(format!("token '{}' found", bop)));
                    }
                } else {
                    return Err(error::AspenError::Unknown(format!("token '{}' found", bop)));
                };
            }
            Token::OpenParen => {
                if let Some(stmt) = statements.last_mut() {
                    match stmt.as_mut() {
                        Statement::Expr(base_expr) => {
                            Expr::modify_into_fn_call(parser, base_expr)?;
                            continue;
                        }
                        Statement::Var(var) => {
                            let Var { value, .. } = var;
                            Expr::modify_into_fn_call(parser, value)?;
                            continue;
                        }
                        _ => (),
                    };
                }

                let expr = Expr::parse_parenthesized(parser)?;
                statements.push(Box::new(Statement::Expr(expr)))
            }
            Token::Range => {
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

                return Err(AspenError::Unknown("token ':' found".to_owned()));
            }
            _ => {}
        }
    }

    if stop_on.is_some() {
        return Err(error::AspenError::Expected(format!(
            "token '{:?}'",
            stop_on
        )));
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
    pub fn statements(&self) -> Container<Statement<'a>> {
        self.body.statements()
    }
    pub fn comments(&self) -> Container<Comment<'a>> {
        self.comments.clone()
    }
    pub fn add_comment(&mut self, comment: Comment<'a>) {
        self.comments.push(Box::new(comment))
    }
    pub fn add_statement(&mut self, statement: Statement<'a>) {
        self.body.add_statement(statement)
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
