use self::{comment::Comment, error::AspenResult, import::Import, value::Value, var::Var};
use crate::{
    lexer::{AspenLexer, Token},
    parser::{import::parse_import_stmt, utils::expect_newline, var::parse_var_stmt},
};
use hashbrown::HashMap;
use logos::Lexer;

pub mod comment;
pub mod error;
mod expr;
pub mod import;
mod macros;
pub mod utils;
pub mod value;
pub mod var;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Var(Var<'a>),
    Import(Import<'a>),
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
    comments: Container<Comment<'s>>,
    statements: Container<Statement<'s>>,
}

pub fn parse_aspen<'s>(parser: &mut AspenParser<'s>) -> AspenResult<()> {
    while let Some(result_token) = parser.lexer.next() {
        let token = result_token?;

        match token {
            Token::Import => {
                let stmt = parse_import_stmt(parser)?;
                parser.statements.push(stmt);
                expect_newline(parser)?;
            }
            Token::Let => {
                let stmt = parse_var_stmt(parser)?;
                parser.statements.push(stmt);
                expect_newline(parser)?;
            }
            Token::LineComment(value)
            | Token::BlockComment(value)
            | Token::MultiLineComment(value) => {
                let start = parser.lexer.span().start;
                let end = parser.lexer.span().end;

                parser.comments.push(Comment::new(value, start, end))
            }
            _ => {
                //todo!
            }
        }
    }

    Ok(())
}

impl<'a> AspenParser<'a> {
    fn new(lexer: Lexer<'a, Token<'a>>) -> Self {
        Self {
            lexer,
            comments: Box::new(vec![]),
            statements: Box::new(vec![]),
        }
    }
    pub fn statements(&self) -> Container<Statement<'a>> {
        self.statements.clone()
    }
    pub fn comments(&self) -> Container<Comment<'a>> {
        self.comments.clone()
    }
}

impl<'a> From<Lexer<'a, Token<'a>>> for AspenParser<'a> {
    fn from(value: Lexer<'a, Token<'a>>) -> Self {
        Self::new(value)
    }
}
