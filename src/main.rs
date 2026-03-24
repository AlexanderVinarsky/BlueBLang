mod token;
mod lexer;

use lexer::Lexer;

fn main() {
    let input = String::from("let x = 123; let y = (6 + 7) * 4; a == b; \"hello\";");
    let mut lexer = Lexer::new(&input);

    let tokens = lexer.tokenize();
    for token in tokens {
        println!{"{}", token.text};
    }
}