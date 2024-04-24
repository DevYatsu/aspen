use dialoguer::{theme::ColorfulTheme, Select};
use logos::Logos;
use parser::error::AspenError;
use std::{fs, time::Instant};

use crate::{
    lexer::Token,
    parser::{import::parse_import_stmt, var::parse_var_stmt},
};

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

    let content = fs::read_to_string(&format!("./aspen/{}", names[choice])).unwrap();
    let mut lexer = Token::lexer(&content);

    let mut statements = vec![];
    let start = Instant::now();
    while let Some(result_token) = lexer.next() {
        let token = result_token?;

        match token {
            Token::Import => statements.push(parse_import_stmt(&mut lexer)?),
            Token::Let => statements.push(parse_var_stmt(&mut lexer)?),
            _ => {
                //todo!
            }
        }
    }

    println!("stmts: {:?}", statements);

    let time_taken = start.elapsed().as_millis();

    println!("Content length: {}", content.len());
    println!("Lexing took {} ms!", time_taken);

    Ok(())
}
