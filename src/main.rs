mod ast;
mod token;
mod lexer;
mod parser;

use parser::Parser;
use lexer::Lexer;

fn main() {
    let cases= [
        "ret 1 + 2 * 3;",
        "ret;",
        "{ let x = 1; x; }",
        "if Purity == true { ret Chloe; } else { ret Max; }",
        "while x < Fang { ret; }",
    ];

    for (i, input) in cases.iter().enumerate() {
        println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        println!("case {}:", i + 1);
        println!("{}", input);

        let mut lexer= Lexer::new(input);
        let tokens= lexer.tokenize();

        let mut parser= Parser::new(tokens);
        let ast= parser.parse_program();

        println!("{:#?}", ast);
        println!();
    }
}