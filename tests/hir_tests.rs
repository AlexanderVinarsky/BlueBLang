
use blueblang::parse_program;
use blueblang::hir::hir;
use blueblang::hir::hir_lowerer::HirLowerer;

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