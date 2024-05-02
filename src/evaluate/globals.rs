use std::io::{stdout, Write};

use super::{error::EvaluateError, value::AspenValue, EvaluateResult};
use hashbrown::HashMap;

// in here are all the global functions defined

pub fn set_up_globals<'a>(hashmap: &mut HashMap<&'a str, AspenValue<'a>>) {
    hashmap.insert(
        "print",
        AspenValue::RustBindFn {
            name: "print",
            code: print,
        },
    );

    hashmap.insert(
        "Array",
        AspenValue::RustBindFn {
            name: "Array",
            code: array,
        },
    );

    hashmap.insert(
        "input",
        AspenValue::RustBindFn {
            name: "input",
            code: input,
        },
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

pub fn array<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    Ok(AspenValue::Array(args))
}

pub fn input<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    let mut user_input = String::new();

    if let Some(prompt) = args.get(0) {
        match prompt {
            AspenValue::Str(prompt_str) => print!("{}", prompt_str),
            _ => return Err(EvaluateError::Custom("Invalid prompt".to_string())),
        }
        let _ = stdout().flush();
    }

    std::io::stdin()
        .read_line(&mut user_input)
        .map_err(|err| EvaluateError::Custom(format!("Error reading input: {}", err)))?;

    user_input.pop(); // Remove newline character

    Ok(AspenValue::Str(user_input))
}
