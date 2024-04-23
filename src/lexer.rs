use logos::Logos;
use rug::{Complete, Float, Integer};

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    #[token("\n")]
    Newline,
    #[token(" ")]
    Space,
    #[token("\t")]
    Carriage,
    #[token(",")]
    Comma,

    #[token("{")]
    OpenBracket,
    #[token("}")]
    CloseBracket,

    #[token("[")]
    OpenBrace,
    #[token("]")]
    CloseBrace,

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
    #[regex(r"(-|\+)?(\d+_\d+)+(\.(\d+((e|E)(-|\+)?\d+)?)?)", |lex| Float::with_val(53, Float::parse(lex.slice()).unwrap(/* the number is valid */)))]
    Float(Float),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*!", |lex| {let raw=lex.slice();&raw[..raw.len()-1]})]
    ObjectKey(&'a str),

    #[regex(r#"=|\+=|-=|\*=|\\=|%="#, |lex| lex.slice())]
    AssignOperator(&'a str),

    #[regex(r#"\+|-|\*\*|\*|\\|%"#, |lex| lex.slice())]
    BinaryOperator(&'a str),
}
