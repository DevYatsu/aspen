use super::{value::AspenValue, EvaluateResult, ValueWrapper};
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
