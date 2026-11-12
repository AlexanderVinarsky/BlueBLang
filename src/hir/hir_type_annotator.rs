use crate::frontend::ast;
use crate::hir::hir;
use crate::hir::hir::{SymbolId, SymbolKind};
use std::collections::HashMap;
use crate::frontend::ast::Item;

pub struct HirTypeAnnotator {
    symbol_types: HashMap<hir::SymbolId, hir::Type>,
    functions: HashMap::<hir::SymbolId, FunctionInfo>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionInfo {
    param_types: Vec<hir::Type>,
    return_type: hir::Type,
}

impl HirTypeAnnotator {
    pub fn annotate_program(&mut self, program: &mut hir::Program) {
        for item in &program.items {
            match item {
                hir::Item::Function(function) => {
                    self.functions.insert(
                        function.symbol_id,
                        FunctionInfo {
                            param_types: function.params.iter().map(|param| param.ty.clone()).collect(),
                            return_type: function.return_type.clone(),
                        },
                    );
                }
            }
        }

        for item in &mut program.items {
            match item {
                hir::Item::Function(function) => {
                    self.annotate_function(function);
                }
            }
        }
    }

    fn annotate_function(&mut self, function: &mut hir::Function) {
        for param in &function.params {
            self.symbol_types.insert(param.symbol_id, param.ty.clone());
        }
        self.annotate_block(&mut function.body);
    }

    fn annotate_block(&mut self, block: &mut hir::Block) {
        for stmt in &mut block.stmts {
            self.annotate_stmt(stmt);
        }
    }


    fn annotate_stmt(&mut self, stmt: &mut hir::Stmt) {
        match stmt {
            hir::Stmt::Let { symbol_id, value, .. } => {
                let ty = self.annotate_expr(value);
                self.symbol_types.insert(*symbol_id, ty);
            }

            hir::Stmt::ExprStmt(expr) => {
                self.annotate_expr(expr);
            }

            hir::Stmt::Block(block) => {
                self.annotate_block(block);
            }

            hir::Stmt::If { condition, then_branch, else_branch } => {
                self.annotate_expr(condition);
                self.annotate_stmt(then_branch);

                if let Some(else_branch) = else_branch {
                    self.annotate_stmt(else_branch);
                }
            }

            hir::Stmt::While { condition, body } => {
                self.annotate_expr(condition);
                self.annotate_stmt(body);
            }

            hir::Stmt::Assign { target, value } => {
                self.annotate_expr(target);
                self.annotate_expr(value);
            }

            hir::Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.annotate_expr(expr);
                }
            }
        }
    }


    fn annotate_expr(&mut self, expr: &mut hir::Expr) -> hir::Type {
        let ty = match &mut expr.kind {
            hir::ExprKind::Number(_) => hir::Type::Int,

            hir::ExprKind::Identifier { symbol_id, .. } => {
                self.symbol_types.get(symbol_id).expect("!unknown symbol type").clone()
            }

            hir::ExprKind::String(_) => hir::Type::String,
            hir::ExprKind::Bool(_) => hir::Type::Bool,

            hir::ExprKind::Binary { left, op, right } => {
                self.annotate_expr(left);
                self.annotate_expr(right);
                Self::binary_result_type(op)
            }

            hir::ExprKind::Unary { op, expr } => {
                self.annotate_expr(expr);
                match op {
                    hir::UnaryOp::Minus => hir::Type::Int,
                    hir::UnaryOp::Bang => hir::Type::Bool,
                }
            }

            hir::ExprKind::Call { callee, args } => {
                for arg in args {
                    self.annotate_expr(arg);
                }

                let symbol_id = match &callee.kind {
                    hir::ExprKind::Identifier { symbol_id, .. } => *symbol_id,
                    _ => unreachable!("!invalid function call target"),
                };

                self.functions.get(&symbol_id).expect("!unknown function").return_type.clone()
            }

            hir::ExprKind::Index { .. } => {
                todo!()
            }

            hir::ExprKind::Member { .. } => {
                todo!()
            }
        };
        expr.ty = Some(ty.clone());
        return ty
    }

    fn binary_result_type(op: &hir::BinaryOp) -> hir::Type {
        match op {
            hir::BinaryOp::Plus
            | hir::BinaryOp::Minus
            | hir::BinaryOp::Star
            | hir::BinaryOp::Slash => hir::Type::Int,

            hir::BinaryOp::EqualEqual
            | hir::BinaryOp::BangEqual
            | hir::BinaryOp::Greater
            | hir::BinaryOp::GreaterEqual
            | hir::BinaryOp::Less
            | hir::BinaryOp::LessEqual
            | hir::BinaryOp::And
            | hir::BinaryOp::Or => hir::Type::Bool,
        }
    }
}