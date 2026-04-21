mod ast;
mod token;
mod lexer;
mod parser;

use parser::Parser;
use lexer::Lexer;

fn main() {
    // Testcases #2
    /*let cases= [
        "ret 1 + 2 * 3;",
        "ret;",
        "{ let x = 1; x; }",
        "if Purity == true { ret Chloe; } else { ret Max; }",
        "while x < Fang { ret; }",
    ];*/

    let cases= [
        "fn main() { ret; }",
        "fn max() { ret squirrel * 3; }",
        "fn fang() { let x = 1; ret x; }",
        "fn chloe() { if True { ret being_cool; } else { ret being_stupid; } }",
        "fn pompidou() { while x < 10 { ret; } }"
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