use blueblang::frontend::semantic::analyze_program;
use blueblang::parse_program;

#[test]
fn accepts_distinct_top_level_functions() {
    let program = parse_program(
        r#"
        fn main() { ret; }
        fn foo() { ret; }
        fn bar() { ret; }
        "#,
    )
        .unwrap();

    let result = analyze_program(&program);

    assert!(
        result.is_ok(),
        "expected semantic success, got: {:?}",
        result.err()
    );
}

#[test]
fn rejects_duplicate_top_level_function_names() {
    let program = parse_program(
        r#"
        fn main() { ret; }
        fn main() { ret; }
        "#,
    )
        .unwrap();

    let result = analyze_program(&program);

    assert!(
        result.is_err(),
        "expected semantic error, but analysis succeeded"
    );
}