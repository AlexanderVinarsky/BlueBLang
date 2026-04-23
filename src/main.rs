use blueblang::parse_program;

fn main() {

    println!("Welcome to BlueBLang!");



    let cases= [
        "fn main() { foo_chloe(); }",
        "fn main() { foo_max(1, 2, 3); }",
        "fn main() { ret foo_chloe(); }",
        "fn main() { let x = arc_bay(); ret x; }",
        "fn main() { foong(guitar); }",
        "fn main() { x = foo_chloe(1); }",
        "fn main() { if true { foo_chloe(); } else { aydar(); } }",

        "fn main() { foo_chloe( ; }",
        "fn main() { foo_chloe 1); }",
        "fn main() { ret foo_chloe(1,); }",
    ];

    for (i, input) in cases.iter().enumerate() {
        println!("case {}:", i + 1);
        println!("{}", input);

        match parse_program(input) {
            Ok(ast) => println!("{:#?}", ast),
            Err(err) => println!("error: {:#?}", err),
        }

        println!();
    }
}