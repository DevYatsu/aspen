use crate::{lexer::Token, parser::parse_aspen};
use dialoguer::{theme::ColorfulTheme, Select};
use logos::Logos;
use parser::error::AspenError;
use std::{env::args, fs, time::Instant};

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
            if arg.starts_with("n=") {
                (arg[2..].parse::<usize>().unwrap(), false)
            } else {
                (1, true)
            }
        })
        .unwrap_or((1, false));

    let content = fs::read_to_string(&format!("./aspen/{}", names[choice]))?.repeat(n);
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
            parse_aspen(&mut parser)?;

            println!("stmts: {:?}", parser.statements());
            println!("comments: {:?}", parser.comments());
            println!("Content length: {}", content.len());
            println!("Lexing took {} ms!", start.elapsed().as_millis());
        }
    };

    Ok(())
}
