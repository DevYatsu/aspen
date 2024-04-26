use std::fmt::Display;

use logos::{Lexer, Logos};
use rug::{Complete, Float, Integer};

pub type AspenLexer<'s> = Lexer<'s, Token<'s>>;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
pub enum Token<'a> {
    #[token("\n")]
    Newline,
    #[regex(r"(\t| )+")]
    Spaces,
    #[token(",")]
    Comma,

    #[regex(r"//[^\n]*", |lex| let raw=lex.slice();*&raw[2..=raw.len()-1].trim())]
    LineComment(&'a str),
    #[regex(r"///[^\n]*", |lex| let raw=lex.slice();*&raw[2..=raw.len()-1].trim())]
    BlockComment(&'a str),
    #[regex(r"/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/", |lex| let raw=lex.slice();&raw[2..=raw.len()-2])]
    MultiLineComment(&'a str),

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

    #[token("->")]
    In,

    #[token("import")]
    Import,
    #[token("let")]
    Let,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("...", priority = 2)]
    SpreadOperator,
    #[token(".", priority = 0)]
    Dot,

    #[regex("@[a-zA-Z_][a-zA-Z0-9_]*", |lex| &lex.slice()[1..])]
    Func(&'a str),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|\\u\{[a-fA-F0-9]+})*""#, |lex| lex.slice())]
    String(&'a str),
    #[token("..", priority = 1)]
    StringSeparator,

    #[regex("true|false", |lex| lex.slice() == "true")]
    Bool(bool),

    #[regex(r"(-|\+)?(\d+(_\d)?)+", |lex| Integer::parse(lex.slice()).unwrap(/* the number is valid */).complete())]
    Int(Integer),
    #[regex(r"(-|\+)?(\d+_\d+)+(\.(\d+((e|E)(-|\+)?\d+)?)?)", |lex| Float::with_val(25, Float::parse(lex.slice()).unwrap(/* the number is valid */)))]
    Float(Float),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*!", |lex| {let raw=lex.slice();&raw[..raw.len()-1]})]
    ObjectKey(&'a str),

    #[regex(r#"=|\+=|-=|\*=|\\=|%="#, |lex| lex.slice())]
    AssignOperator(&'a str),

    #[regex(r#"\+|-|\*\*|\*|\\|%"#, |lex| lex.slice())]
    BinaryOperator(&'a str),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LexingError();

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to lex: Unexpected token!")
    }
}
