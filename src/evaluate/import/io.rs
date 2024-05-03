use std::io::Write;

use super::super::{error::EvaluateError, value::AspenValue, EvaluateResult};
use hashbrown::HashMap;

pub fn random_module<'a>() -> AspenValue<'a> {
    let mut hashmap = HashMap::new();

    hashmap.insert(
        "input",
        AspenValue::RustBindFn {
            name: "input",
            code: input,
        },
    );

    AspenValue::Object(hashmap)
}

pub fn input<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    let mut user_input = String::new();

    if let Some(prompt) = args.get(0) {
        match prompt {
            AspenValue::Str(prompt_str) => print!("{}", prompt_str),
            _ => return Err(EvaluateError::Custom("Invalid prompt".to_string())),
        }
        let _ = std::io::stdout().flush();
    }

    std::io::stdin()
        .read_line(&mut user_input)
        .map_err(|err| EvaluateError::Custom(format!("Error reading input: {}", err)))?;

    user_input.pop(); // Remove newline character

    Ok(AspenValue::Str(user_input))
}
