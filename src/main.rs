use dialoguer::{theme::ColorfulTheme, Select};
use logos::Logos;
use std::fs;
mod lexer;

fn main() {
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
    let mut lex = lexer::Token::lexer(&content);

    while let Some(token) = lex.next() {
        println!("{:?}", token)
    }
}
