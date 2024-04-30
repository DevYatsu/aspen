use hashbrown::HashMap;
use rug::{Float, Integer};

use crate::parser::{
    error::AspenResult,
    func::{Argument, Func},
    utils::Block,
    value::Value,
    var::Var,
    Container, Expr, Statement,
};

use self::error::EvaluateError;

pub mod error;

#[derive(Debug, Clone)]
pub struct AspenTable<'a> {
    functions: HashMap<&'a str, AspenFn<'a>>,
    vars: HashMap<&'a str, AspenValue<'a>>,
}

#[derive(Debug, Clone)]
pub struct AspenFn<'a> {
    args: Vec<Box<Argument<'a>>>,
    body: Box<Block<'a>>,
}

#[derive(Debug, Clone)]
pub enum AspenValue<'a> {
    Nil,
    Str(String),
    Bool(bool),

    Int(Integer),
    Float(Float),

    Array(Container<AspenValue<'a>>),
    Object(HashMap<&'a str, AspenValue<'a>>),
}

pub type EvaluateResult<T> = Result<T, EvaluateError>;

pub fn execute(stmts: Container<Statement<'_>>) -> AspenResult<()> {
    let mut table = AspenTable::new();

    for stmt in stmts.into_iter() {
        match *stmt {
            Statement::Func(f) => {
                table.insert_fn(f)?;
            }
            Statement::Var(var) => {
                table.insert_var(var)?;
            }
            _ => (),
        }
    }

    println!("{:?}", table);

    Ok(())
}

impl<'a> AspenTable<'a> {
    pub fn new() -> Self {
        AspenTable {
            functions: HashMap::new(),
            vars: HashMap::new(),
        }
    }

    pub fn get_var(&self, name: &'a str) -> EvaluateResult<&AspenValue<'a>> {
        let opt_value = self.vars.get(name);

        match opt_value {
            Some(value) => Ok(value),
            None => Err(EvaluateError::UnknownVar(name.to_owned())),
        }
    }

    pub fn interpret_value(&self, expr: Expr<'a>) -> EvaluateResult<AspenValue<'a>> {
        match expr {
            Expr::Value(val) => Ok(val.into()),
            Expr::Id(name) => {
                let value = self.get_var(name)?.to_owned();
                Ok(value)
            }
            _ => todo!(),
        }
    }

    pub fn insert_fn(&mut self, f: Func<'a>) -> EvaluateResult<()> {
        let Func {
            name,
            arguments,
            body,
        } = f;

        self.functions.insert(
            name,
            AspenFn {
                args: arguments,
                body,
            },
        );

        Ok(())
    }

    pub fn insert_var(&mut self, v: Var<'a>) -> EvaluateResult<()> {
        let Var { variables, value } = v;

        let true_value = self.interpret_value(*value)?;

        match variables.len() {
            1 => {
                self.vars.insert(variables[0], true_value);
            }
            _ => todo!(),
        }

        Ok(())
    }
}

impl<'a> From<Value<'a>> for AspenValue<'a> {
    fn from(value: Value<'a>) -> Self {
        match value {
            Value::Nil => AspenValue::Nil,
            Value::Int(i) => AspenValue::Int(i),
            Value::Str(s) => AspenValue::Str(s.to_owned()),
            Value::Bool(b) => AspenValue::Bool(b),
            Value::Float(f) => AspenValue::Float(f),
        }
    }
}
