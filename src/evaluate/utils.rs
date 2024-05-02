use super::{error::EvaluateError, value::AspenValue, EvaluateResult};
use rug::Integer;

pub fn extract_range<'a>(value: AspenValue<'a>) -> EvaluateResult<(usize, usize, usize)> {
    match value {
        AspenValue::Range { start, end, step } => {
            let start = match *start {
                AspenValue::Int(i) => i,
                _ => {
                    return Err(EvaluateError::Custom(
                        "Start of the range must be an integer".to_string(),
                    ))
                }
            };

            let end = match *end {
                AspenValue::Int(i) => i,
                _ => {
                    return Err(EvaluateError::Custom(
                        "End of the range must be an integer".to_string(),
                    ))
                }
            };

            let step = match step {
                Some(val) => match *val {
                    AspenValue::Int(i) => i,
                    _ => {
                        return Err(EvaluateError::Custom(
                            "Step of the range must be an integer".to_string(),
                        ))
                    }
                },
                None => Integer::from(1),
            };

            if start >= end {
                return Err(EvaluateError::Custom(
                    "Start of the range must be less than end of the range".to_string(),
                ));
            }
            if end > usize::MAX {
                return Err(EvaluateError::Custom(format!(
                    "A range end index cannot exceed {}!!",
                    usize::MAX
                )));
            }
            if start < usize::MIN {
                return Err(EvaluateError::Custom(format!(
                    "A range end index cannot be less than {}!!",
                    usize::MIN
                )));
            }

            if step < usize::MIN {
                return Err(EvaluateError::Custom(format!(
                    "A range step cannot be less than {}!!",
                    usize::MIN
                )));
            }
            if step > usize::MAX {
                return Err(EvaluateError::Custom(format!(
                    "A range step cannot be more than {}!!",
                    usize::MAX
                )));
            }

            let standart_deviation = end.to_owned() - start.to_owned();

            if standart_deviation > 1_000_000 {
                return Err(EvaluateError::Custom(
                    "A range start and end index cannot have a gap of more than 1_000_000!!"
                        .to_string(),
                ));
            }

            let start = start.to_usize().unwrap();
            let end = end.to_usize().unwrap();
            let step = step.to_usize().unwrap();

            Ok((start, end, step))
        }
        _ => {
            return Err(EvaluateError::Custom(
                "The argument is expected to be of type range".to_string(),
            ))
        }
    }
}
