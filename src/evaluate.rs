use hashbrown::HashMap;
use rug::{float::OrdFloat, Integer};

use crate::parser::{func::Func, value::Value, var::Var, Container, Expr, Statement};

use self::{error::EvaluateError, func::AspenFn, types::AspenType, value::AspenValue};

pub mod error;
pub mod func;
pub mod types;
mod value;

#[derive(Debug, Clone)]
pub struct AspenTable<'a> {
    values: HashMap<&'a str, AspenValue<'a>>,
}

pub type EvaluateResult<T> = Result<T, EvaluateError>;

pub fn evaluate(stmts: Container<Statement<'_>>) -> EvaluateResult<()> {
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
            values: HashMap::new(),
        }
    }

    pub fn get_value(&self, name: &'a str) -> EvaluateResult<&AspenValue<'a>> {
        let opt_value = self.values.get(name);

        match opt_value {
            Some(value) => Ok(value),
            None => Err(EvaluateError::UndefinedIdentifier(name.to_owned())),
        }
    }

    pub fn is_identifier_used(&self, ident: &'a str) -> bool {
        self.get_value(ident).is_ok()
    }

    pub fn evaluate_expr(&self, expr: Expr<'a>) -> EvaluateResult<AspenValue<'a>> {
        match expr {
            Expr::Value(val) => Ok(val.into()),
            Expr::Id(name) => {
                let value = self.get_value(name)?.to_owned();
                Ok(value)
            }
            Expr::Import(name) => {
                todo!()
            }
            Expr::FuncCall { callee, args } => {
                let func_name = match self.evaluate_expr(*callee)? {
                    AspenValue::Str(s) => s,
                    x => return Err(EvaluateError::OnlyFuncsCanBeCalled(x.to_string())),
                };

                let func = self.get_value(&func_name)?;

                match func {
                    AspenValue::Func(f) => {}
                    _ => return Err(EvaluateError::IdentifierIsNotValidFn(func_name)),
                }
                todo!()
            }
            Expr::StringConcatenation { left, right } => {
                let left = self.evaluate_expr(*left)?;
                let right = self.evaluate_expr(*right)?;

                match (left, right) {
                    (AspenValue::Str(l), AspenValue::Str(r)) => Ok(AspenValue::Str(l + &r)),

                    (AspenValue::Str(_), x) => {
                        return Err(EvaluateError::InvalidType {
                            expected: AspenType::String,
                            found: x.into(),
                        })
                    }
                    (x, _) => {
                        return Err(EvaluateError::InvalidType {
                            expected: AspenType::String,
                            found: x.into(),
                        })
                    }
                }
            }
            Expr::Parenthesized(expr) => self.evaluate_expr(*expr),
            Expr::Range { start, end, step } => {
                let step = match step {
                    None => None,
                    Some(expr) => Some(self.evaluate_expr(*expr).map(Box::new)?),
                };

                Ok(AspenValue::Range {
                    start: Box::new(self.evaluate_expr(*start)?),
                    end: Box::new(self.evaluate_expr(*end)?),
                    step,
                })
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

        if self.is_identifier_used(name) {
            return Err(EvaluateError::IdentifierAlreadyUsed(name.to_owned()));
        }

        self.values.insert(
            name,
            AspenValue::Func(AspenFn {
                args: arguments,
                body,
                name,
            }),
        );

        Ok(())
    }

    pub fn insert_var(&mut self, v: Var<'a>) -> EvaluateResult<()> {
        let Var { variables, value } = v;

        for name in variables.iter() {
            if self.is_identifier_used(name) {
                return Err(EvaluateError::IdentifierAlreadyUsed(name.to_string()));
            }
        }

        let true_value = self.evaluate_expr(*value)?;

        match variables.len() {
            1 => {
                self.insert_value(variables[0], true_value);
            }
            _ => todo!(),
        }

        Ok(())
    }

    pub fn insert_value(&mut self, name: &'a str, value: AspenValue<'a>) -> EvaluateResult<()> {
        if self.is_identifier_used(name) {
            return Err(EvaluateError::IdentifierAlreadyUsed(name.to_string()));
        }

        self.values.insert(name, value);

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
