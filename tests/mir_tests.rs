use blueblang::parse_program;
use blueblang::hir::hir_lowerer::HirLowerer;
use blueblang::hir::hir_type_annotator::HirTypeAnnotator;
use blueblang::mir::mir;
use blueblang::mir::mir_lowerer::MirLowerer;

fn lower(source: &str) -> mir::Program {
    let ast = parse_program(source).unwrap();
    let mut hir = HirLowerer::new().lower_program(&ast);

    let mut annotator = HirTypeAnnotator::new();
    annotator.annotate_program(&mut hir);

    let mut lowerer = MirLowerer::new();
    lowerer.lower_program(hir)
}

#[test]
fn lowers_let_binary_and_assign() {
    let program = lower(r#"
        fn main() {
            let chloe = 1;
            chloe = chloe + 2;
            ret;
        }
    "#);

    let function = &program.functions[0];
    assert_eq!(function.blocks.len(), 1);
    assert_eq!(function.locals.len(), 2);
    assert!(function.locals[0].symbol_id.is_some());
    assert!(function.locals[1].symbol_id.is_none());

    let block = &function.blocks[0];
    assert!(matches!(block.instructions.as_slice(), [
        mir::Instruction::Copy {
            destination: mir::Place::Local(mir::LocalId(0)),
            source: mir::Operand::Constant(mir::Constant::Int(1))
        },
        mir::Instruction::Binary {
            destination: mir::Place::Local(mir::LocalId(1)),
            op: mir::BinaryOp::Add,
            lhs: mir::Operand::Local(mir::LocalId(0)),
            rhs: mir::Operand::Constant(mir::Constant::Int(2))
        },
        mir::Instruction::Copy {
            destination: mir::Place::Local(mir::LocalId(0)),
            source: mir::Operand::Local(mir::LocalId(1))
        },
    ]));
    assert!(matches!(block.terminator, mir::Terminator::Return(None)));
}

#[test]
fn lowers_if_else_to_join() {
    let program = lower(r#"
        fn main() {
            let x = 0;
            if (true) { x = 1; } else { x = 2; }
            ret;
        }
    "#);

    let function = &program.functions[0];
    let mir::Terminator::Branch { then_block, else_block, .. } = &function.blocks[0].terminator else {
        panic!("!expected branch");
    };

    let then_bb = &function.blocks[then_block.0 as usize];
    let else_bb = &function.blocks[else_block.0 as usize];

    assert!(matches!(then_bb.instructions.as_slice(), [mir::Instruction::Copy {
        destination: mir::Place::Local(mir::LocalId(0)),
        source: mir::Operand::Constant(mir::Constant::Int(1))
    }]));
    assert!(matches!(else_bb.instructions.as_slice(), [mir::Instruction::Copy {
        destination: mir::Place::Local(mir::LocalId(0)),
        source: mir::Operand::Constant(mir::Constant::Int(2))
    }]));

    let mir::Terminator::Goto(then_join) = &then_bb.terminator else {
        panic!("!expected goto from then branch");
    };
    let mir::Terminator::Goto(else_join) = &else_bb.terminator else {
        panic!("!expected goto from else branch");
    };

    assert_eq!(then_join, else_join);
    assert!(matches!(
        function.blocks[then_join.0 as usize].terminator,
        mir::Terminator::Return(None)
    ));
}

#[test]
fn no_join_when_both_if_branches_return() {
    let program = lower(r#"
        fn main() {
            if (true) { ret; } else { ret; }
        }
    "#);

    let function = &program.functions[0];
    assert_eq!(function.blocks.len(), 3);

    let mir::Terminator::Branch { then_block, else_block, .. } = &function.blocks[0].terminator else {
        panic!("!expected branch");
    };
    assert!(matches!(
        function.blocks[then_block.0 as usize].terminator,
        mir::Terminator::Return(None)
    ));
    assert!(matches!(
        function.blocks[else_block.0 as usize].terminator,
        mir::Terminator::Return(None)
    ));
}

#[test]
fn lowers_while_with_back_edge() {
    let program = lower(r#"
        fn main() {
            let x = 0;
            while (x < 3) {
                x = x + 1;
            }
            ret;
        }
    "#);

    let function = &program.functions[0];
    let mir::Terminator::Goto(condition_id) = &function.blocks[0].terminator else {
        panic!("!expected goto to condition block");
    };

    let condition_bb = &function.blocks[condition_id.0 as usize];
    assert!(matches!(condition_bb.instructions.as_slice(), [
        mir::Instruction::Binary { op: mir::BinaryOp::Lt, .. }
    ]));

    let mir::Terminator::Branch { then_block, else_block, .. } = &condition_bb.terminator else {
        panic!("!expected branch from condition block");
    };
    let body_bb = &function.blocks[then_block.0 as usize];
    assert!(matches!(body_bb.instructions.as_slice(), [
        mir::Instruction::Binary { op: mir::BinaryOp::Add, .. },
        mir::Instruction::Copy { .. },
    ]));
    assert!(matches!(
        &body_bb.terminator,
        mir::Terminator::Goto(back_to) if back_to == condition_id
    ));
    assert!(matches!(
        function.blocks[else_block.0 as usize].terminator,
        mir::Terminator::Return(None)
    ));
}

#[test]
fn return_stops_lowering_following_statements() {
    let program = lower(r#"
        fn main() {
            ret;
            let unreachable = 67;
        }
    "#);

    let function = &program.functions[0];
    assert_eq!(function.blocks.len(), 1);
    assert!(function.blocks[0].instructions.is_empty());
    assert!(function.locals.is_empty());
    assert!(matches!(function.blocks[0].terminator, mir::Terminator::Return(None)));
}