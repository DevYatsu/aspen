use self::func::Func;
use self::utils::Block;
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
    Expr(Expr<'a>),
}

crate::impl_from_for!(Expr, Statement);

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Value(Value<'a>),

    Array(Vec<Box<Expr<'a>>>),

    Object(HashMap<&'a str, Expr<'a>>),

    Id(&'a str),

    SpeadId(&'a str),
}

pub type Container<T> = Box<Vec<T>>;

#[derive(Debug, Clone)]
pub struct AspenParser<'s> {
    pub lexer: AspenLexer<'s>,
    body: Block<'s>,
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
    let mut statements = Box::new(vec![]);
    let mut comments = Box::new(vec![]);

    while let Some(result_token) = parser.lexer.next() {
        let token = result_token?;

        match token {
            Token::Import => {
                let stmt = Import::parse(parser)?;
                statements.push(stmt);
                expect_newline(parser)?;
            }
            Token::Let => {
                let stmt = Var::parse(parser)?;
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
                let stmt = Func::parse(parser, name)?;
                statements.push(stmt);
            }
            _ if stop_on.is_some() && &token == stop_on.as_ref().unwrap() => {
                println!("{:?}", statements);
                return Ok(Block::new(statements, comments));
            }
            // Token::Identifier(id) => {
            //     if id == "print" {
            //         let next = next_jump_multispace(parser)?;

            //         println!("{:?}", next)
            //     } else {
            //     }
            // }
            Token::Nil
            | Token::Bool(_)
            | Token::Float(_)
            | Token::Int(_)
            | Token::OpenBrace
            | Token::OpenBracket
            | Token::SpreadOperator
            | Token::String(_) => {
                if let Ok(ex) = Expr::parse_with_token(parser, token) {
                    statements.push(ex.into())
                }
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

    Ok(Block::new(statements, comments))
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
