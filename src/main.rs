use blueblang::parse_program;

fn main() {
    println!("Welcome to BlueBLang!");

    let cases = [
        "fn main() { arr[i] = x; }",
        "fn main() { arr[i + 1] = foo.bar; }",
        "fn main() { foo.bar = x; }",
        "fn main() { foo.bar[i] = y; }",
        "fn main() { foo.bar()[i] = z; }",
        "fn main() { arr[] = x; }",
        "fn main() { foo. = x; }",
        "fn main() { foo.bar = ; }",
        "fn main() { arr[i = x; }",
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
