use blueblang::parse_program;
use blueblang::hir::hir_lowerer::HirLowerer;

fn main() {
    println!("Welcome to BlueBLang!");

    let cases = [
        "fn main() { arr[i] = x; }",
    ];


    for (i, input) in cases.iter().enumerate() {
        println!("{}", input);

        match parse_program(input) {
            Ok(ast) => {
                let hir = HirLowerer::new().lower_program(&ast);
                println!("AST:\n{:#?}", ast);
                println!("HIR:\n{:#?}", hir);
            }
            Err(err) => println!("error: {:#?}", err),
        }

        println!();
    }
}
