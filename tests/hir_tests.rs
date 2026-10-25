
use blueblang::parse_program;
use blueblang::hir::hir;
use blueblang::hir::hir_lowerer::HirLowerer;

fn lower(source: &str) -> hir::Program {
    let ast = parse_program(source).unwrap();
    let mut lowerer = HirLowerer::new();
    lowerer.lower_program(&ast)
}

fn identifier_id(expr: &hir::Expr) -> hir::SymbolId {
    let hir::Expr::Identifier { symbol_id, .. } = expr else {
        panic!("!expected Identifier");
    };
    *symbol_id
}

fn called_id(stmt: &hir::Stmt) -> hir::SymbolId {
    let hir::Stmt::ExprStmt(hir::Expr::Call { callee, .. }) = stmt else {
        panic!("!expected Call");
    };
    identifier_id(callee)
}

#[test]
fn lowers_for_to_while() {
    let source = r#"
        fn main() {
            for (let i = 0, i < 10, i = i + 1) {
                let x = i;
            }
        }
    "#;

    let ast = parse_program(source).unwrap();

    let mut lowerer = HirLowerer::new();
    let hir = lowerer.lower_program(&ast);

    println!("AST:\n{:#?}", ast);
    println!("HIR:\n{:#?}", hir);

    let hir::Item::Function(function) = &hir.items[0];

    // For -> Block
    let hir::Stmt::Block(block) = &function.body.stmts[0] else {
        panic!("expected Block after for lowering");
    };

    // init
    assert!(matches!(
        block.stmts.first(),
        Some(hir::Stmt::Let { .. })
    ));

    // while
    assert!(matches!(
        block.stmts.get(1),
        Some(hir::Stmt::While { .. })
    ));
}

#[test]
fn resolves_function_calls() {
    let program = lower(r#"
        fn main() {
            kozyava();
        }

        fn kozyava() {
            kozyava();
        }
    "#);

    let hir::Item::Function(main) = &program.items[0];
    let hir::Item::Function(kozyava) = &program.items[1];

    assert_ne!(main.symbol_id, kozyava.symbol_id);
    assert_eq!(called_id(&main.body.stmts[0]), kozyava.symbol_id);
    assert_eq!(called_id(&kozyava.body.stmts[0]), kozyava.symbol_id);
}

#[test]
fn resolves_parameter_shadowing() {
    let program = lower(r#"
        fn main() {}

        fn foo(chloe: int) {
            {
                let chloe = chloe;
                chloe;
            }
            chloe;
        }
    "#);

    let hir::Item::Function(foo) = &program.items[1];
    let param_id = foo.params[0].symbol_id;

    let hir::Stmt::Block(inner) = &foo.body.stmts[0] else {
        panic!("!expected Block");
    };

    let hir::Stmt::Let { symbol_id, value, .. } = &inner.stmts[0] else {
        panic!("!expected Let");
    };

    assert_eq!(identifier_id(value), param_id);
    assert_ne!(*symbol_id, param_id);

    let hir::Stmt::ExprStmt(inner_use) = &inner.stmts[1] else {
        panic!("!expected ExprStmt");
    };

    assert_eq!(identifier_id(inner_use), *symbol_id);

    let hir::Stmt::ExprStmt(outer_use) = &foo.body.stmts[1] else {
        panic!("!expected ExprStmt");
    };

    assert_eq!(identifier_id(outer_use), param_id);
}

#[test]
fn resolves_for_variable() {
    let program = lower(r#"
        fn main() {
            for (let i = 0, i < 10, i = i + 1) {}
        }
    "#);

    // let i = 0;
    //
    // while i < 10 {
    //     {}
    //     i = i + 1;
    // }

    let hir::Item::Function(main) = &program.items[0];

    let hir::Stmt::Block(block) = &main.body.stmts[0] else {
        panic!("!expected Block");
    };

    let hir::Stmt::Let { symbol_id, .. } = &block.stmts[0] else {
        panic!("!expected Let");
    };

    let hir::Stmt::While { condition, body } = &block.stmts[1] else {
        panic!("!expected While");
    };

    let hir::Expr::Binary { left, .. } = condition else {
        panic!("!expected Binary condition");
    };

    assert_eq!(identifier_id(left), *symbol_id);

    let hir::Stmt::Block(while_body) = body.as_ref() else {
        panic!("!expected While body");
    };

    let hir::Stmt::Assign { target, .. } = &while_body.stmts[1] else {
        panic!("!expected Assign");
    };

    assert_eq!(identifier_id(target), *symbol_id);
}