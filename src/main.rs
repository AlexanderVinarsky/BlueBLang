mod ast;
mod token;
mod lexer;
mod parser;

use parser::Parser;
use lexer::Lexer;

fn main() {
    let input = String::from("
    1 + 2 * 3 < 10;
    ");

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    println!("{:#?}", ast);
}