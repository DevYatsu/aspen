use crate::parser::operator::{AssignOperator, BinaryOperator};
use logos::{Lexer, Logos};
pub use rug::{Complete, Float, Integer};
use std::fmt::Display;

pub type AspenLexer<'s> = Lexer<'s, Token<'s>>;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
pub enum Token<'a> {
    #[token("\n")]
    Newline,
    #[regex(r"(\t|\r| )+")]
    Spaces,
    #[token(",")]
    Comma,
    #[token(";")]
    SemiColon,

    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,

    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,

    #[regex(r"->|in")]
    In,

    #[token("nil")]
    Nil,

    #[token("if")]
    If,
    #[regex(r"other|otherwise")]
    /// Represents the elseif variant keyword of an if: can be either 'other' or 'otherwise'
    Other,
    #[token("else")]
    Else,

    #[regex(r"\$import|\$imp|\$")]
    Import,
    #[token("return")]
    Return,
    #[token("let")]
    Let,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("...", priority = 2)]
    SpreadOperator,
    #[token("..", priority = 1)]
    StringSeparator,
    #[token(".", priority = 0)]
    Dot,
    #[token(":")]
    Range,

    #[regex(r"//[^\n]*", |lex| let raw=lex.slice();raw[2..=raw.len()-1].trim())]
    LineComment(&'a str),
    #[regex(r"///[^\n]*", |lex| let raw=lex.slice();raw[2..=raw.len()-1].trim())]
    DocComment(&'a str),
    #[regex(r"/\*([^*]|\*[^/])*\*/", |lex| let raw=lex.slice();&raw[2..=raw.len()-3])]
    MultiLineComment(&'a str),

    #[regex("@[a-zA-Z_][a-zA-Z0-9_]*", |lex| &lex.slice()[1..])]
    Func(&'a str),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|\\u\{[a-fA-F0-9]+})*""#, |lex| lex.slice())]
    String(&'a str),

    #[regex("true|false", |lex| lex.slice() == "true")]
    Bool(bool),

    #[regex(r"-?\d+(_?\d)*", |lex| Integer::parse(lex.slice()).unwrap(/* the number is valid */).complete(), priority = 5)]
    Int(Integer),
    #[regex(r"-?\d+(_?\d)*(\.\d+)([eE][-+]?\d+)?", |lex| Float::with_val(25, Float::parse(lex.slice()).unwrap(/* the number is valid */)), priority = 4)]
    Float(Float),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*!", |lex| {let raw=lex.slice();&raw[..raw.len()-1]})]
    ObjectKey(&'a str),

    #[regex(r#"=|\+=|-=|\*=|/=|%="#, |lex| {
        match  lex.slice() {
            "=" => AssignOperator::Equal,
            "+=" => AssignOperator::Plus,
            "-=" => AssignOperator::Sub,
            "*=" => AssignOperator::Times,
            "/=" => AssignOperator::Divide,
            "%=" => AssignOperator::Modulo,
            _ => unreachable!(),
        }
    })]
    AssignOperator(AssignOperator),

    #[regex(r#"\+|-|\*\*|\*|/|%|!=|==|>=|>|<=|<|&&|\|\|"#, |lex| {
        match lex.slice() {
            "+" => BinaryOperator::Plus,
            "-" => BinaryOperator::Sub,
            "*" => BinaryOperator::Times,
            "**" => BinaryOperator::Exponent,
            "/" => BinaryOperator::Divide,
            "%" => BinaryOperator::Modulo,
            "==" => BinaryOperator::Equal,
            "!=" => BinaryOperator::NotEqual,
            ">"=> BinaryOperator::GreaterThan,
            ">="=> BinaryOperator::GreaterThanOrEqual,
            "<"=> BinaryOperator::LessThan,
            "<="=> BinaryOperator::LessThanOrEqual,
            "&&" => BinaryOperator::And,
            "||" => BinaryOperator::Or,
            _ => unreachable!(),
        }
    })]
    BinaryOperator(BinaryOperator),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LexingError();

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to lex: Unexpected token!")
    }
}
