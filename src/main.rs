mod token;
mod lexer;

use lexer::Lexer;

fn main() {
    let input = String::from("a=(x==y)");
    let mut lexer = Lexer::new(&input);

    let tokens = lexer.tokenize();
    for token in tokens {
        println!{"{}", token.text};
    }
}