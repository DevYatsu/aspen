use super::{error::EvaluateError, utils::extract_range, value::AspenValue, EvaluateResult};
use hashbrown::HashMap;
use rug::{float::OrdFloat, Float, Integer};

pub mod math;
