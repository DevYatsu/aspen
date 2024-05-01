use crate::{
    evaluate::{error::EvaluateError, AspenTable},
    parser::{func::Argument, utils::Block},
};

use super::{AspenValue, EvaluateResult};

#[derive(Debug, Clone, PartialEq)]
pub struct AspenFn<'a> {
    pub args: Vec<Argument<'a>>,
    pub body: Box<Block<'a>>,
    pub name: &'a str,
}

impl<'a> AspenFn<'a> {
    pub fn call(&self, args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
        // there can only be one spread argument, it's ensured by the parser
        let minimum_num = self.args.iter().filter(|a| a.is_spread == false).count();
        let found_num = args.len();

        let has_spread_arg = minimum_num != self.args.len();

        if found_num < minimum_num {
            return Err(EvaluateError::NotEnoughArgs {
                expected_num: minimum_num,
                found: found_num,
            });
        }

        if !has_spread_arg && found_num > minimum_num {
            return Err(EvaluateError::TooMuchArgs {
                expected_num: minimum_num,
                found: found_num,
            });
        }

        let mut ctx = self.init_ctx(args, has_spread_arg)?;
        let result = ctx.evaluate_block(self.body.statements())?;

        todo!()
    }

    fn init_ctx(
        &self,
        args: Vec<AspenValue<'a>>,
        has_spread_arg: bool,
    ) -> EvaluateResult<AspenTable<'a>> {
        let mut fn_ctx = AspenTable::new();

        if has_spread_arg {
            for (i, arg) in self.args.iter().enumerate() {
                if !arg.is_spread {
                    fn_ctx.insert_value(arg.identifier, args[i].clone())?;
                } else {
                    let spread_args = args.iter().skip(i - 1).cloned().collect::<Vec<_>>();
                    fn_ctx.insert_value(arg.identifier, AspenValue::Array(spread_args))?;
                    break;
                }
            }
        } else {
            for (arg, value) in self.args.iter().zip(args.into_iter()) {
                fn_ctx.insert_value(arg.identifier, value)?;
            }
        }

        Ok(fn_ctx)
    }
}
