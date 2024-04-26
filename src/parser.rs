use self::func::parse_fn_statement;
use self::utils::Block;
use self::{
    comment::Comment, error::AspenResult, func::Func, import::Import, value::Value, var::Var,
};
use crate::{
    lexer::{AspenLexer, Token},
    parser::{import::parse_import_stmt, utils::expect_newline, var::parse_var_stmt},
};
use hashbrown::HashMap;
use logos::Lexer;

pub mod comment;
pub mod error;
mod expr;
pub mod func;
pub mod import;
mod macros;
pub mod utils;
pub mod value;
pub mod var;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Var(Var<'a>),
    Import(Import<'a>),
    Func(Func<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Value(Value<'a>),

    Array(Vec<Box<Expr<'a>>>),

    Object(HashMap<&'a str, Expr<'a>>),

    Id(&'a str),
}

pub type Container<T> = Box<Vec<T>>;

#[derive(Debug, Clone)]
pub struct AspenParser<'s> {
    pub lexer: AspenLexer<'s>,
    body: Block<'s>,
}

pub fn parse_aspen<'s>(parser: &mut AspenParser<'s>) -> AspenResult<()> {
    let (result, _) = parse_block(parser, false)?;

    parser.body = result;

    Ok(())
}

/// Parses code into a block of statements.
///
/// If stop_on_error is set to true, the parsing will stop when a unknow token is found, can be used e.g `}` for a function block ending.
pub fn parse_block<'s>(
    parser: &mut AspenParser<'s>,
    stop_on_error: bool,
) -> AspenResult<(Block<'s>, Option<Token<'s>>)> {
    let mut statements = Box::new(vec![]);
    let mut comments = Box::new(vec![]);

    while let Some(result_token) = parser.lexer.next() {
        let token = result_token?;

        match token {
            Token::Import => {
                let stmt = parse_import_stmt(parser)?;
                statements.push(stmt);
                expect_newline(parser)?;
            }
            Token::Let => {
                let stmt = parse_var_stmt(parser)?;
                statements.push(stmt);
                expect_newline(parser)?;
            }
            Token::LineComment(value)
            | Token::BlockComment(value)
            | Token::MultiLineComment(value) => {
                let start = parser.lexer.span().start;
                let end = parser.lexer.span().end;

                comments.push(Comment::new(value, start, end))
            }
            Token::Func(name) => {
                let stmt = parse_fn_statement(parser, name)?;
                statements.push(stmt);
            }
            _ => {
                if stop_on_error {
                    return Ok((Block::new(statements, comments), Some(token)));
                }
                //todo!
            }
        }
    }

    Ok((Block::new(statements, comments), None))
}

impl<'a> AspenParser<'a> {
    fn new(lexer: Lexer<'a, Token<'a>>) -> Self {
        Self {
            lexer,
            body: Block::default(),
        }
    }
    pub fn statements(&self) -> Container<Statement<'a>> {
        self.body.statements()
    }
    pub fn comments(&self) -> Container<Comment<'a>> {
        self.body.comments()
    }
}

impl<'a> From<Lexer<'a, Token<'a>>> for AspenParser<'a> {
    fn from(value: Lexer<'a, Token<'a>>) -> Self {
        Self::new(value)
    }
}
