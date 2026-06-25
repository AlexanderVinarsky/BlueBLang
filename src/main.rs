use blueblang::parse_program;

fn main() {

    println!("Welcome to BlueBLang!");



    let cases= [
        "fn main() { foo.bar; }",
        "fn main() { foo.bar(); }",
        "fn main() { arr[i]; }",
        "fn main() { arr[i + 1]; }",
        "fn main() { foo.bar[i]; }",
        "fn main() { foo.bar()[i]; }",
        "fn main() { foo.bar(x, y); }",
        "fn main() { foo(bar)[i]; }",
        "fn main() { ret foo.bar()[i]; }",

        "fn main() { foo.; }",
        "fn main() { arr[]; }",
        "fn main() { arr[i; }",
        "fn main() { foo.(x); }",
        "fn main() { foo.bar(,); }",
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