use super::{value::AspenValue, EvaluateResult};
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
        "Err",
        AspenValue::RustBindFn {
            name: "Err",
            code: error,
        },
    );

    hashmap.insert(
        "Array",
        AspenValue::RustBindFn {
            name: "Array",
            code: array,
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
