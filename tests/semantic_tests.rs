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
    ).unwrap();

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
    ).unwrap();

    let result = analyze_program(&program);

    assert!(
        result.is_err(),
        "expected semantic error, but analysis succeeded"
    );
}

#[test]
fn accepts_distinct_function_params() {
    let src = r#"
        fn pow(base, exp) {}
        fn main() {}
    "#;
    let program = parse_program(src).unwrap();
    let result = analyze_program(&program);
    assert!(
        result.is_ok(),
        "semantic analysis failed: {:?}",
        result.err()
    );
}

#[test]
fn rejects_duplicate_function_params() {
    let src = r#"
        fn pow(x, x) {}
    "#;
    let program = parse_program(src).unwrap();
    let result = analyze_program(&program);
    assert!(result.is_err(), "expected duplicate parameter error");
}

#[test]
fn rejects_unknown_function_call() {
    let src = r#"
        fn main() {
            unknown(1);
        }
    "#;

    let program = parse_program(src).unwrap();
    let result = analyze_program(&program);

    assert!(result.is_err(), "expected unknown function error");
}

#[test]
fn rejects_wrong_function_argument_count() {
    let src = r#"
        fn pow(base, exp) {}

        fn main() {
            pow(2);
        }
    "#;

    let program = parse_program(src).unwrap();
    let result = analyze_program(&program);

    assert!(result.is_err(), "expected wrong argument count error");
}

#[test]
fn rejects_member_as_call_target() {
    let src = r#"
        fn main() {
            foo.bar(1);
        }
    "#;

    let program = parse_program(src).unwrap();
    let result = analyze_program(&program);

    assert!(result.is_err(), "expected invalid call target error");
}

#[test]
fn rejects_missing_main_function() {
    let src = r#"
        fn kozyavka() {}
    "#;

    let program = parse_program(src).unwrap();
    let result = analyze_program(&program);

    assert!(result.is_err(), "expected missing main function error");
}

#[test]
fn rejects_main_with_parameters() {
    let src = r#"
        fn main(x) {}
    "#;

    let program = parse_program(src).unwrap();
    let result = analyze_program(&program);

    assert!(result.is_err(), "expected invalid main signature error");
}

#[test]
fn accepts_declared_local_variable() {
    let program = parse_program(
        r#"
        fn main() {
            let x = 1;
            x;
        }
        "#,
    ).unwrap();

    let result = analyze_program(&program);

    assert!(
        result.is_ok(),
        "expected semantic success, got: {:?}",
        result.err()
    );
}

#[test]
fn accepts_function_parameter_as_local_variable() {
    let program = parse_program(
        r#"
        fn main() {}

        fn foo(x) {
            x;
        }
        "#,
    ).unwrap();

    let result = analyze_program(&program);

    assert!(
        result.is_ok(),
        "expected semantic success, got: {:?}",
        result.err()
    );
}

#[test]
fn rejects_variable_used_outside_its_block_scope() {
    let program = parse_program(
        r#"
        fn main() {
            {
                let aydar = 1;
            }
            aydar;
        }
        "#,
    ).unwrap();

    let result = analyze_program(&program);

    assert!(
        result.is_err(),
        "expected unknown variable error"
    );
}

#[test]
fn accepts_assignment_to_declared_variable() {
    let program = parse_program(
        r#"
        fn main() {
            let x = 1;
            x = 2;
        }
        "#,
    ).unwrap();

    let result = analyze_program(&program);

    assert!(result.is_ok(), "expected semantic success, got: {:?}", result.err());
}

#[test]
fn rejects_assignment_to_undeclared_variable() {
    let program = parse_program(
        r#"
        fn main() {
            x = 1;
        }
        "#,
    ).unwrap();

    let result = analyze_program(&program);

    assert!(result.is_err(), "expected assignment to undeclared variable error");
}
