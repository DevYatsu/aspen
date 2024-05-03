use std::{fs, io::Write};

use super::super::{error::EvaluateError, value::AspenValue, EvaluateResult};
use hashbrown::HashMap;

pub fn module<'a>() -> AspenValue<'a> {
    let mut hashmap = HashMap::new();

    hashmap.insert(
        "read",
        AspenValue::RustBindFn {
            name: "read",
            code: read_file,
        },
    );

    hashmap.insert(
        "write",
        AspenValue::RustBindFn {
            name: "write",
            code: write_file,
        },
    );
    hashmap.insert(
        "append",
        AspenValue::RustBindFn {
            name: "append",
            code: append_file,
        },
    );
    AspenValue::Object(hashmap)
}

pub fn read_file<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    if args.len() != 1 {
        return Err(EvaluateError::Custom(
            "'read' function expects 1 argument: a file name".to_string(),
        ));
    }

    let file_name = match args[0].to_owned() {
        AspenValue::Str(n) => n,
        _ => {
            return Err(EvaluateError::Custom(
                "'read' argument is not of type 'String'".to_string(),
            ))
        }
    };

    let result = match fs::read_to_string(file_name) {
        Ok(s) => s.into(),
        Err(s) => AspenValue::Error(s.to_string()),
    };

    Ok(result)
}

pub fn write_file<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    if args.len() != 2 {
        return Err(EvaluateError::Custom(
            "'write' function expects 2 arguments: a file name and a content".to_string(),
        ));
    }

    let file_name = match args[0].to_owned() {
        AspenValue::Str(n) => n,
        _ => {
            return Err(EvaluateError::Custom(
                "'write' first argument is not of type 'String'".to_string(),
            ))
        }
    };

    let data = match args[1].to_owned() {
        AspenValue::Str(d) => d,
        _ => {
            return Err(EvaluateError::Custom(
                "'write' second argument is not of type 'String'".to_string(),
            ))
        }
    };

    match fs::write(&file_name, data) {
        Ok(_) => Ok(AspenValue::Nil),
        Err(err) => Ok(AspenValue::Error(err.to_string())),
    }
}

pub fn append_file<'a>(args: Vec<AspenValue<'a>>) -> EvaluateResult<AspenValue<'a>> {
    if args.len() != 2 {
        return Err(EvaluateError::Custom(
            "'append' function expects 2 arguments: a file name and a content".to_string(),
        ));
    }

    let file_name = match args[0].to_owned() {
        AspenValue::Str(n) => n,
        _ => {
            return Err(EvaluateError::Custom(
                "'append' first argument is not of type 'String'".to_string(),
            ))
        }
    };

    let data = match args[1].to_owned() {
        AspenValue::Str(d) => d,
        _ => {
            return Err(EvaluateError::Custom(
                "'append' second argument is not of type 'String'".to_string(),
            ))
        }
    };

    match fs::OpenOptions::new().append(true).open(&file_name) {
        Ok(mut file) => match file.write_all(data.as_bytes()) {
            Ok(_) => Ok(AspenValue::Nil),
            Err(err) => Ok(AspenValue::Error(err.to_string())),
        },
        Err(err) => Ok(AspenValue::Error(err.to_string())),
    }
}
