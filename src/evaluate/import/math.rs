use super::super::{error::EvaluateError, utils::extract_range, value::AspenValue, EvaluateResult};
use hashbrown::HashMap;
use rug::{float::OrdFloat, Float, Integer};

pub fn random_module<'a>() -> AspenValue<'a> {
    let mut hashmap = HashMap::new();

    hashmap.insert(
        "random",
        AspenValue::RustBindFn {
            name: "random",
            code: random,
        },
    );

    hashmap.insert(
        "random_int",
        AspenValue::RustBindFn {
            name: "random_int",
            code: random_int,
        },
    );

    hashmap.insert(
        "shuffle",
        AspenValue::RustBindFn {
            name: "shuffle",
            code: shuffle,
        },
    );

    AspenValue::Object(hashmap)
}

use rand::prelude::*;

pub fn random<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    if args.len() != 0 {
        return Err(EvaluateError::Custom(
            "random function expects no argument".to_string(),
        ));
    }

    let mut rng = rand::thread_rng();
    let f: f64 = rng.gen();
    let random_number = Float::with_val(18, f);
    Ok(AspenValue::Float(OrdFloat::from(random_number)))
}

pub fn random_int<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    if args.len() != 1 {
        return Err(EvaluateError::Custom(
            "shuffle function expects 1 argument: a range of ints".to_string(),
        ));
    }

    let (start, end, step) = extract_range(args[0].to_owned())?;
    let base_range = start..=end;
    let expected_range = (start..=end).step_by(step);

    let mut rng = rand::thread_rng();
    let mut num: usize = rng.gen_range(base_range.clone());

    while expected_range.clone().any(|x| x == num) {
        num = rng.gen_range(base_range.clone());
    }

    Ok(AspenValue::Int(Integer::from(num)))
}

pub fn shuffle<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    if args.len() != 1 {
        return Err(EvaluateError::Custom(
            "shuffle function expects 1 argument: a range of ints".to_string(),
        ));
    }

    let (start, end, step) = extract_range(args[0].to_owned())?;
    let mut nums: Vec<usize> = (start..=end).step_by(step).collect();

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    Ok(AspenValue::Array(
        nums.into_iter()
            .map(|x| AspenValue::Int(Integer::from(x)))
            .collect(),
    ))
}
