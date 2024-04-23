use logos::Logos;
mod lexer;

fn main() {
    let content = std::fs::read_to_string("fn.aspen").unwrap();
    let mut lex = lexer::Token::lexer(&content);

    while let Some(token) = lex.next() {
        println!("{:?}", token)
    }
}
