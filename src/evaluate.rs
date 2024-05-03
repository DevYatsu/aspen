use self::{
    error::EvaluateError, func::AspenFn, globals::set_up_globals, types::AspenType,
    value::AspenValue,
};
use crate::parser::{
    func::Func, operator::AssignOperator, return_stmt::Return, value::Value, var::Var, Container,
    Expr, Statement,
};
use hashbrown::HashMap;
use rug::Integer;

pub mod error;
pub mod func;
mod globals;
mod import;
pub mod types;
mod utils;
mod value;

#[derive(Debug, Clone)]
pub struct AspenTable<'a> {
    values: HashMap<&'a str, AspenValue<'a>>,
}

pub type EvaluateResult<T> = Result<T, EvaluateError>;

impl<'a> AspenTable<'a> {
    pub fn new() -> Self {
        AspenTable {
            values: HashMap::new(),
        }
    }

    pub fn global() -> Self {
        let mut values = HashMap::new();
        set_up_globals(&mut values);
        Self { values }
    }

    pub fn evaluate_block(
        &mut self,
        stmts: Container<Statement<'a>>,
    ) -> EvaluateResult<AspenValue<'a>> {
        for stmt in stmts.into_iter() {
            match *stmt {
                Statement::Func(f) => {
                    self.insert_fn(f)?;
                }
                Statement::Var(var) => {
                    self.insert_var(var)?;
                }
                Statement::Return(Return(value)) => return Ok(self.evaluate_expr(*value)?),
                Statement::Expr(expr) => match *expr {
                    Expr::Assign {
                        target,
                        operator,
                        value,
                        // no need to pay attention to it cause we are at the end of the ctx an assignment is useless
                    } => {
                        let name = match *target {
                            Expr::Id(name) => name,
                            Expr::ObjIndexing { indexed, indexer } => {
                                todo!()
                            }
                            expr => {
                                return Err(EvaluateError::Custom(format!(
                                    "Value can only be assigned to variable, not value '{}'",
                                    expr
                                )));
                            }
                        };

                        match operator {
                            AssignOperator::Equal => {
                                self.update_value(name, self.evaluate_expr(*value)?)?;
                            }
                            _ => todo!(),
                        };
                    }
                    expr => {
                        self.evaluate_expr(expr)?;
                    }
                },
                _ => todo!(),
            }
        }

        println!("{:?}", self);

        Ok(AspenValue::Nil)
    }

    pub fn get_ref_value(&self, name: &'a str) -> EvaluateResult<&AspenValue<'a>> {
        let opt_value = self.values.get(name);

        match opt_value {
            Some(value) => Ok(value),
            None => Err(EvaluateError::UndefinedIdentifier(name.to_owned())),
        }
    }

    pub fn get_value(&self, name: String) -> EvaluateResult<AspenValue<'a>> {
        let opt_value = self.values.get(name.as_str());

        match opt_value {
            Some(value) => Ok(value.clone()),
            None => Err(EvaluateError::UndefinedIdentifier(name.to_owned())),
        }
    }

    pub fn is_identifier_used(&self, ident: &'a str) -> bool {
        self.get_ref_value(ident).is_ok()
    }

    pub fn evaluate_expr(&self, expr: Expr<'a>) -> EvaluateResult<AspenValue<'a>> {
        match expr {
            Expr::Value(val) => Ok(val.into()),
            Expr::Id(name) => {
                let value = self.get_ref_value(name)?.to_owned();
                Ok(value)
            }
            Expr::PropagatedFailible(expr) => {}
            Expr::Import(name) => {
                todo!()
            }
            Expr::FuncCall { callee, args } => {
                let func_name = match *callee {
                    Expr::Id(id) => id,
                    Expr::ObjIndexing { indexed, indexer } => {
                        todo!()
                    }
                    x => return Err(EvaluateError::OnlyFuncsCanBeCalled(x.to_string())),
                };

                let func = self.get_value(func_name.to_owned())?;

                let args_result: Result<Vec<_>, _> =
                    args.into_iter().map(|e| self.evaluate_expr(*e)).collect();
                let args = args_result?;

                match func {
                    AspenValue::Func(f) => return Ok(f.call(args)?),
                    AspenValue::RustBindFn { code, name } => {
                        let r = code(args)?;
                        return Ok(r);
                    }
                    _ => return Err(EvaluateError::IdentifierIsNotValidFn(func_name.to_owned())),
                }
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
            Expr::Array(exprs) => {
                let mut args = Vec::with_capacity(exprs.len());

                for expr in exprs.into_iter() {
                    match *expr {
                        Expr::SpeadId(id) => match self.get_value(id.to_owned())? {
                            AspenValue::Array(vals) => args.extend(vals),
                            AspenValue::Range { start, end, step } => match (*start, *end) {
                                (AspenValue::Int(i), AspenValue::Int(i2)) => {
                                    let vals: Vec<_> = (i.to_i128().unwrap()
                                        ..=i2.to_i128().unwrap())
                                        .map(|i| AspenValue::Int(Integer::from(i)))
                                        .collect();
                                    args.extend(vals)
                                }
                                (AspenValue::Str(s), AspenValue::Str(s2)) => {
                                    if s.len() == 1 && s2.len() == 1 {
                                        let vals: Vec<_> = (s.chars().next().unwrap()
                                            ..=s2.chars().next().unwrap())
                                            .map(|s| AspenValue::Str(s.to_string()))
                                            .collect();

                                        args.extend(vals)
                                    } else {
                                        return Err(EvaluateError::Custom(format!(
                                            "A range can only be generated from two numbers or two characters, \"{}\" and \"{}\" must be one character long",
                                            s, s2
                                        )));
                                    }
                                }
                                (a, b) => {
                                    return Err(EvaluateError::Custom(format!(
                                        "A range cannot be created from type '{}' and type '{}'",
                                        AspenType::from(a),
                                        AspenType::from(b)
                                    )));
                                }
                            },
                            _ => {
                                return Err(EvaluateError::Custom(format!(
                                    "Only arrays can be spread, '{}' is not a valid array",
                                    id
                                )))
                            }
                        },
                        e => args.push(self.evaluate_expr(e)?),
                    }
                }

                Ok(AspenValue::Array(args))
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
                self.insert_value(variables[0], true_value)?;
            }
            _ => todo!(),
        }

        Ok(())
    }

    pub fn update_value(&mut self, name: &'a str, value: AspenValue<'a>) -> EvaluateResult<()> {
        let previous_value = self.values.insert(name, value);

        if previous_value.is_none() {
            return Err(EvaluateError::Custom(format!(
                "Cannot assign value to undefined variable '{}'",
                name
            )));
        }

        match previous_value.unwrap() {
            AspenValue::Func(_) => {
                return Err(EvaluateError::Custom(format!(
                    "Cannot assign value to function '{}'",
                    name
                )));
            }
            _ => (),
        };

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
