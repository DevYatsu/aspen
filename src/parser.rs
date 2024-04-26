use self::{comment::Comment, error::AspenResult, import::Import, value::Value, var::Var};
use crate::{
    lexer::Token,
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

pub type AspenLexer<'s> = Lexer<'s, Token<'s>>;

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

pub fn parse_aspen<'s>(lexer: &mut AspenLexer<'s>) -> AspenResult<Vec<Statement<'s>>> {
    let mut statements = vec![];
    let mut directives = vec![];

    while let Some(result_token) = lexer.next() {
        let token = result_token?;

        match token {
            Token::Import => {
                statements.push(parse_import_stmt(lexer)?);
                expect_newline(lexer)?;
            }
            Token::Let => {
                statements.push(parse_var_stmt(lexer)?);
                expect_newline(lexer)?;
            }
            Token::LineComment(value)
            | Token::BlockComment(value)
            | Token::MultiLineComment(value) => {
                let start = lexer.span().start;
                let end = lexer.span().end;

                directives.push(Comment::new(value, start, end))
            }
            _ => {
                //todo!
            }
        }
    }

    Ok(statements)
}
