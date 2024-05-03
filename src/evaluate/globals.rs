use std::io::Write;

use super::{error::EvaluateError, value::AspenValue, EvaluateResult, ValueWrapper};
use hashbrown::HashMap;

// in here are all the global functions defined

pub fn set_up_globals<'a>(hashmap: &mut HashMap<&'a str, ValueWrapper<'a>>) {
    hashmap.insert(
        "print",
        ValueWrapper::CurrentContext(AspenValue::RustBindFn {
            name: "print",
            code: print,
        }),
    );
    hashmap.insert(
        "input",
        ValueWrapper::CurrentContext(AspenValue::RustBindFn {
            name: "input",
            code: input,
        }),
    );

    hashmap.insert(
        "Err",
        ValueWrapper::CurrentContext(AspenValue::RustBindFn {
            name: "Err",
            code: error,
        }),
    );

    hashmap.insert(
        "Array",
        ValueWrapper::CurrentContext(AspenValue::RustBindFn {
            name: "Array",
            code: array,
        }),
    );
}

pub fn print<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    for (i, arg) in args.iter().enumerate() {
        print!("{arg}");

        if i != args.len() - 1 {
            print!(", ")
        }
    }

    print!("\n");

    Ok(AspenValue::Nil)
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

// Function named 'Err'
pub fn error<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    let mut result = String::new();

    for (i, arg) in args.iter().enumerate() {
        result.push_str("{arg}");

        if i != args.len() - 1 {
            result.push_str(", ")
        }
    }

    result.push_str("\n");

    Ok(AspenValue::Error(result))
}

pub fn array<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    Ok(AspenValue::Array(args))
}
