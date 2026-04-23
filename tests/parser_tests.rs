use blueblang::parse_program;

fn assert_ok_group(group:&str, cases:&[&str]) {
    for (i, src) in cases.iter().enumerate() {
        let result= parse_program(src);
        assert!(
            result.is_ok(),
            "group: {}\ncase #{}\nexpected: success\nsource:\n{}\nerror:\n{:?}",
            group,
            i+1,
            src,
            result.err()
        );
    }
}

fn assert_err_group(group:&str, cases:&[&str]) {
    for (i, src) in cases.iter().enumerate() {
        let result= parse_program(src);
        assert!(
            result.is_err(),
            "group: {}\ncase #{}\nexpected: error\nsource:\n{}\nactual:\n{:?}",
            group,
            i+1,
            src,
            result.ok()
        );
    }
}





// TESTS



// RETURNS AND FNS

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



// ASSIGNMENT

#[test]
fn parses_assignment_case_group() {
    let cases= [
        "fn main() { x = 1; }",
        "fn main() { x = a + b * c; }",
        "fn main() { while max_age < 100 { squirrel_photos = squirrel_photos + 1; } }",
        "fn main() { aydar == 1; }",
        "fn main() { let x = 1; x = x + 2; ret x; }",
        "fn main() { if true { chloe = cool; } else { chloe = stupid; } }",
    ];

    assert_ok_group("assignment cases", &cases);
}

#[test]
fn rejects_assignment_case_group() {
    let cases= [
        "fn main() { maxs_intelligence = ; }",
        "fn fang() { while x < 10 { x = x + 1 } }",
    ];

    assert_err_group("assignment error cases", &cases);
}



// FN CALL

#[test]
fn parses_call_case_group() {
    let cases= [
        "fn main() { foo_chloe(); }",
        "fn main() { foo_max(1, 2, 3); }",
        "fn main() { ret foo_chloe(); }",
        "fn main() { let x = arc_bay(); ret x; }",
        "fn main() { foong(guitar); }",
        "fn main() { x = foo_chloe(1); }",
        "fn main() { if true { foo_chloe(); } else { aydar(); } }",
    ];

    assert_ok_group("call cases", &cases);
}

#[test]
fn rejects_call_case_group() {
    let cases= [
        "fn main() { foo_chloe( ; }",
        "fn main() { foo_chloe 1); }",
        "fn main() { ret foo_chloe(1,); }",
    ];

    assert_err_group("call error cases", &cases);
}