use hashbrown::HashMap;

use self::{import::Import, value::Value, var::Var};

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
