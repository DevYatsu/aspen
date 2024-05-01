use crate::{errors::build_error, lexer::Token, parser::parse_aspen};
use dialoguer::{theme::ColorfulTheme, Select};
use logos::Logos;
use parser::error::AspenError;
use std::{env::args, fs, time::Instant};

mod errors;
mod evaluate;
mod lexer;
mod parser;

fn main() -> Result<(), AspenError> {
    let names: Vec<_> = fs::read_dir("./aspen/")
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which file to read?")
        .items(&names)
        .interact()
        .unwrap();

    let args = args().collect::<Vec<String>>();
    let (n, see_tokens) = args
        .get(1)
        .map(|arg| {
            if let Some(v) = arg.strip_prefix("n=") {
                (v.parse::<usize>().unwrap(), false)
            } else {
                (1, true)
            }
        })
        .unwrap_or((1, false));

    let file_name = format!("./aspen/{}", names[choice]);
    let content = fs::read_to_string(&file_name)?.repeat(n);
    let mut parser: parser::AspenParser<'_> = Token::lexer(&content).into();

    match see_tokens {
        true => {
            for i in parser.lexer {
                println!("{:?}", i)
            }
            println!("Content length: {}", content.len());
        }
        false => {
            let start = Instant::now();
            if let Err(e) = parse_aspen(&mut parser) {
                build_error(parser.lexer.source(), e, &file_name)
            };

            println!("stmts: {:?}", parser.statements());
            println!("comments: {:?}", parser.comments());
            println!("Content length: {}", content.len());
            println!("Lexing+Parsing took {} ms!", start.elapsed().as_millis());

            let start = Instant::now();
            // evaluate(parser.statements())?;
            // println!("Executing took {} ms!", start.elapsed().as_millis());
        }
    };

    Ok(())
}

// fn run_with_f_name() -> Result<(), AspenError> {
//     let args = args().collect::<Vec<String>>();
//     let mut name = args.get(1);

//     if let None = name {
//         println!("Missing a file name");
//         return Ok(());
//     }

//     let content = fs::read_to_string(name.take().unwrap())?.repeat(50000);
//     let mut parser: parser::AspenParser<'_> = Token::lexer(&content).into();

//     let start = Instant::now();
//     parse_aspen(&mut parser)?;

//     // println!("stmts: {:?}", parser.statements());
//     // println!("comments: {:?}", parser.comments());
//     println!("Content length: {}", content.len());
//     println!("Lexing took {} ms!", start.elapsed().as_millis());

//     Ok(())
// }
