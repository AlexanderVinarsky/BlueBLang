use blueblang::ast::*;
use blueblang::parse_program;

fn assert_program_ok(src:&str) {
    let result= parse_program(src);
    assert!(
        result.is_ok(),
        "expected parse success\nsource:\n{}\nerror:\n{:?}",
        src,
        result.err()
    );
}

fn assert_ok_group(group:&str, cases:&[&str]) {
    for (i, src) in cases.iter().enumerate() {
        let result= parse_program(src);
        assert!(
            result.is_ok(),
            "group: {}\ncase #{}\nsource:\n{}\nerror:\n{:?}",
            group,
            i+1,
            src,
            result.err()
        );
    }
}


#[test]
fn parses_stmt_case_group() {
    let cases= [
        "fn main() { ret 1 + 2 * 3; }",
        "fn main() { ret; }",
        "fn main() { { let x = 1; x; } }",
        "fn main() { if Purity == true { ret Chloe; } else { ret Max; } }",
        "fn main() { while x < Fang { ret; } }",
    ];

    assert_ok_group("stmt cases", &cases);
}


#[test]
fn parses_function_case_group() {
    let cases= [
        "fn main() { ret; }",
        "fn max() { ret squirrel * 3; }",
        "fn fang() { let x = 1; ret x; }",
        "fn chloe() { if true { ret being_cool; } else { ret being_stupid; } }",
        "fn pompidou() { while x < 10 { ret; } }",
    ];

    assert_ok_group("function cases", &cases);
}