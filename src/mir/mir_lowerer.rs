use crate::hir::hir;
use crate::mir::mir;
use std::collections::HashMap;
use crate::hir::hir::{Expr, Stmt};
use crate::mir::mir::LocalId;

pub struct MirLowerer {}

struct PendingBlock {
    instructions: Vec<mir::Instruction>,
    terminator: Option<mir::Terminator>
}

struct FunctionBuilder { // NB for terminators
    locals: Vec<mir::LocalDecl>,
    blocks: Vec<PendingBlock>,
    current_block: Option<mir::BlockId>,
    symbol_to_local: HashMap<hir::SymbolId, mir::LocalId>
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            blocks: vec![
                PendingBlock {
                    instructions: Vec::new(),
                    terminator: None
                }
            ],
            current_block: Some(mir::BlockId(0)),
            symbol_to_local: HashMap::new()
        }
    }

    pub fn add_local(&mut self, ty: mir::Type, symbol_id: Option<hir::SymbolId>) -> mir::LocalId {
        let local_id = mir::LocalId(self.locals.len() as u32);
        self.locals.push(mir::LocalDecl{ty, symbol_id});
        if let Some(symbol_id) = symbol_id {
            self.symbol_to_local.insert(symbol_id, local_id);
        }
        return local_id;
    }

    pub fn new_block(&mut self) -> mir::BlockId {
        let block_id = mir::BlockId(self.blocks.len() as u32);
        self.blocks.push(PendingBlock{instructions: Vec::new(), terminator: None});
        return block_id;
    }

    pub fn switch_to_block(&mut self, block_id: mir::BlockId) {
        let block = self.blocks.get(block_id.0 as usize).expect("!BB does not exist"); //might be removved after BB deletion solution
        assert!(block.terminator.is_none(), "!cant switch to terminated BB");
        self.current_block = Some(block_id);
    }

    fn finish(self) -> (Vec<mir::LocalDecl>, Vec<mir::BasicBlock>) {
        let blocks = self.blocks.into_iter().map(|block| {
                mir::BasicBlock {
                    instructions: block.instructions,
                    terminator: block.terminator.expect("!BB has no terminator")
                }
            })
            .collect();
        (self.locals, blocks)
    }

    fn add_instruction(&mut self, instr: mir::Instruction) {
        let block_id = self.current_block.expect("!no active basic block");
        let block = &mut self.blocks[block_id.0 as usize];
        assert!(block.terminator.is_none(), "!cant emit instruction after terminator");
        block.instructions.push(instr);
    }

    fn add_terminator(&mut self, terminator: mir::Terminator) {
        let block_id = self.current_block.take().expect("!no active basic block");
        let block = &mut self.blocks[block_id.0 as usize];
        assert!(block.terminator.is_none(), "!basic block already has a terminator");
        block.terminator = Some(terminator);
    }
}

impl MirLowerer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lower_program(&mut self, program: hir::Program) -> mir::Program {
        let mut functions = Vec::<mir::Function>::new();
        for item in &program.items {
            match item {
                hir::Item::Function(function) => {
                    functions.push(self.lower_function(&function));
                }
            }
        }
        return mir::Program { functions, symbols: program.symbols};
    }

    fn lower_function(&mut self, function: &hir::Function) -> mir::Function {
        let name = function.name.clone();
        let symbol_id = function.symbol_id;

        let mut params = Vec::<LocalId>::new();
        let mut builder = FunctionBuilder::new();
        for param in &function.params {
            let local_id = builder.add_local(
                self.lower_type(&param.ty),
                Some(param.symbol_id)
            );
            params.push(local_id);
        }
        let return_type = self.lower_type(&function.return_type);
        self.lower_block(&function.body, &mut builder);
        let (locals, blocks) = builder.finish();
        return mir::Function {name, symbol_id, params, return_type, locals, blocks};
    }

    fn lower_type(&self, ty: &hir::Type) -> mir::Type {
        match ty {
            hir::Type::Int => mir::Type::Int,
            hir::Type::Bool => mir::Type::Bool,
            hir::Type::String => mir::Type::String,
            hir::Type::Unit => mir::Type::Unit,
        }
    }

    fn lower_block(&mut self, block: &hir::Block, builder: &mut FunctionBuilder) {
        for stmt in &block.stmts {
            if builder.current_block.is_none() {
                break;
            }
            self.lower_stmt(stmt, builder);
        }
    }

    fn lower_while(&mut self, condition: &Expr, body: &Stmt, builder: &mut FunctionBuilder) {
        let condition_block = builder.new_block();
        let body_block      = builder.new_block();
        let exit_block      = builder.new_block();

        builder.add_terminator(mir::Terminator::Goto(condition_block));
        builder.switch_to_block(body_block);
        let condition = self.lower_expr(condition, builder);
        builder.add_terminator(mir::Terminator::Branch {condition, then_block: body_block, else_block: exit_block});

        self.lower_stmt(body, builder);

        if builder.current_block.is_some() {
            builder.add_terminator(mir::Terminator::Goto(condition_block));
        }
        builder.switch_to_block(exit_block);
    }

    fn lower_if(&mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: Option<&Stmt>,
        builder: &mut FunctionBuilder
    ) {
        let condition = self.lower_expr(condition, builder);
        let then_block = builder.new_block();

        if let Some(else_branch) = else_branch {
            let else_block = builder.new_block();

            builder.add_terminator(mir::Terminator::Branch {condition, then_block, else_block});

            builder.switch_to_block(then_block);
            self.lower_stmt(then_branch, builder);
            let then_end = builder.current_block;

            builder.switch_to_block(else_block);
            self.lower_stmt(else_branch, builder);
            let else_end = builder.current_block;

            if then_end.is_none() && else_end.is_none() {
                return;
            }

            let join_block = builder.new_block();

            if let Some(block) = then_end {
                builder.switch_to_block(block);
                builder.add_terminator(mir::Terminator::Goto(join_block));
            }

            if let Some(block) = else_end {
                builder.switch_to_block(block);
                builder.add_terminator(mir::Terminator::Goto(join_block));
            }

            builder.switch_to_block(join_block);
        }

        else {
            let join_block = builder.new_block();
            builder.add_terminator(mir::Terminator::Branch {condition, then_block, else_block: join_block});
            builder.switch_to_block(then_block);
            self.lower_stmt(then_branch, builder);

            if builder.current_block.is_some() {
                builder.add_terminator(mir::Terminator::Goto(join_block));
            }

            builder.switch_to_block(join_block);
        }
    }

    fn lower_unary_op(&mut self, op: &hir::UnaryOp) -> mir::UnaryOp {
        match op {
            hir::UnaryOp::Bang => mir::UnaryOp::Not,
            hir::UnaryOp::Minus => mir::UnaryOp::Neg
        }
    }

    fn lower_binary_op(&mut self, op: &hir::BinaryOp) -> mir::BinaryOp {
        match op {
            hir::BinaryOp::Plus => mir::BinaryOp::Add,
            hir::BinaryOp::Minus => mir::BinaryOp::Sub,
            hir::BinaryOp::Star => mir::BinaryOp::Mul,
            hir::BinaryOp::Slash => mir::BinaryOp::Div,

            hir::BinaryOp::EqualEqual => mir::BinaryOp::Eq,
            hir::BinaryOp::BangEqual => mir::BinaryOp::Ne,

            hir::BinaryOp::Greater => mir::BinaryOp::Gt,
            hir::BinaryOp::GreaterEqual => mir::BinaryOp::Ge,
            hir::BinaryOp::Less => mir::BinaryOp::Lt,
            hir::BinaryOp::LessEqual => mir::BinaryOp::Le,

            hir::BinaryOp::And => mir::BinaryOp::And,
            hir::BinaryOp::Or => mir::BinaryOp::Or,
        }
    }

    fn lower_expr(&mut self, expr: &hir::Expr, builder: &mut FunctionBuilder) -> mir::Operand {
        match &expr.kind {
            hir::ExprKind::Number(number) => {
                mir::Operand::Constant(
                    mir::Constant::Int(number.parse::<i64>().unwrap())
                )
            }

            hir::ExprKind::Identifier { symbol_id, .. } => {
                let local_id = builder.symbol_to_local[symbol_id];
                mir::Operand::Local(local_id)
            }

            hir::ExprKind::String(string) => {
                mir::Operand::Constant(
                    mir::Constant::String(string.clone())
                )
            }

            hir::ExprKind::Bool(bool) => {
                mir::Operand::Constant(
                    mir::Constant::Bool(bool.clone())
                )
            }

            hir::ExprKind::Binary { left, op, right } => {
                let lhs = self.lower_expr(left, builder);
                let rhs = self.lower_expr(right, builder);
                let ty = self.lower_type(expr.ty.as_ref().expect("!untyped HIR expression"));
                let destination = builder.add_local(ty, None);

                builder.add_instruction(mir::Instruction::Binary {
                    destination: mir::Place::Local(destination),
                    op: self.lower_binary_op(op),
                    lhs,
                    rhs
                });

                mir::Operand::Local(destination)
            }

            hir::ExprKind::Unary { op, expr: operand_expr } => {
                let operand = self.lower_expr(operand_expr, builder);
                let ty = self.lower_type(
                    expr.ty.as_ref().expect("!untyped HIR expression")
                );
                let destination = builder.add_local(ty, None);

                builder.add_instruction(mir::Instruction::Unary {
                    destination: mir::Place::Local(destination),
                    op: self.lower_unary_op(op),
                    operand
                });
                mir::Operand::Local(destination)
            }

            hir::ExprKind::Call { callee, args } => {
                todo!()
            }

            hir::ExprKind::Index { .. } => {
                todo!()
            }

            hir::ExprKind::Member { .. } => {
                todo!()
            }
        }
    }

    fn lower_stmt(&mut self, stmt: &hir::Stmt, builder: &mut FunctionBuilder) {
        match stmt {
            hir::Stmt::Let { symbol_id, value, .. } => {
                let source = self.lower_expr(value, builder);
                let ty = self.lower_type(
                    value.ty.as_ref().expect("!untyped HIR expression")
                );

                let local_id = builder.add_local(ty, Some(*symbol_id));

                builder.add_instruction(mir::Instruction::Copy {
                    destination: mir::Place::Local(local_id),
                    source
                });
            }

            hir::Stmt::ExprStmt(expr) => {
                self.lower_expr(expr, builder);
            }

            hir::Stmt::Block(block) => {
                self.lower_block(block, builder);
            }

            hir::Stmt::If {condition, then_branch, else_branch} => {
                self.lower_if(condition, then_branch, else_branch.as_deref(), builder);
            }

            hir::Stmt::While {condition, body} => {
                self.lower_while(condition, body, builder);
            }

            hir::Stmt::Assign { target, value } => {
                todo!(); //self.lower_assign(target, value, builder);
            }

            hir::Stmt::Return(expr) => {
                let value = expr.as_ref().map(|expr| self.lower_expr(expr, builder));
                builder.add_terminator(mir::Terminator::Return(value));
            }
        }
    }
}