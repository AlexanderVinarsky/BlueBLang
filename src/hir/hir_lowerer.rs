use crate::frontend::ast;
use crate::hir::hir;
use crate::hir::hir::{SymbolId, SymbolKind};
use std::collections::HashMap;

pub struct HirLowerer {
    scopes: Vec<HashMap<String, SymbolId>>,
    symbols: Vec<hir::Symbol>,
    next_symbol_id: u32
}

impl HirLowerer {
    pub fn new() -> Self {
        Self {
            scopes: Vec::<HashMap<String, SymbolId>>::new(),
            symbols: Vec::<hir::Symbol>::new(),
            next_symbol_id: 0
        }
    }

    pub fn lower_program(mut self, program: &ast::Program) -> hir::Program {
        self.scopes.push(HashMap::new()); //global scope!

        for item in &program.items {
            match item {
                ast::Item::Function(function) => {
                    self.declare(&function.name, hir::SymbolKind::Function);
                }
            }
        }

        let items = program.items.iter().map(|item| self.lower_item(item)).collect();
        self.scopes.pop();

        hir::Program {
            items,
            symbols: self.symbols,
        }
    }

    pub fn lower_item(&mut self, item: &ast::Item) -> hir::Item {
        match item {
            ast::Item::Function(function) => hir::Item::Function(self.lower_function(function))
        }
    }

    pub fn lower_function(&mut self, function: &ast::Function) -> hir::Function {
        let symbol_id = self.resolve(&function.name).expect("!function must be declared");
        self.scopes.push(HashMap::new());

        let name = function.name.clone();
        let params = function.params.iter().map(|param| self.lower_param(param)).collect();
        let return_type = self.lower_type_annotation(&function.return_type);
        let body = self.lower_block(&function.body);

        self.scopes.pop();

        return hir::Function { name, params, symbol_id, return_type, body };
    }

    pub fn lower_param(&mut self, param: &ast::Param) -> hir::Param {
        let symbol_id = self.declare(&param.name, SymbolKind::Variable);

        hir::Param {
            name: param.name.clone(),
            symbol_id,
            type_annotation: self.lower_type_annotation(&param.type_annotation)
        }
    }

    pub fn lower_block(&mut self, block: &ast::Block) -> hir::Block {
        self.scopes.push(HashMap::new());
        let stmts = block.stmts.iter().map(|stmt| self.lower_stmt(stmt)).collect();
        self.scopes.pop();
        return hir::Block { stmts }
    }

    pub fn lower_stmt(&mut self, stmt: &ast::Stmt) -> hir::Stmt {
        match stmt {
            ast::Stmt::Let { name, value } => {
                let value = self.lower_expr(value);
                let symbol_id = self.declare(name, hir::SymbolKind::Variable);

                hir::Stmt::Let {
                    name: name.clone(),
                    symbol_id,
                    value
                }
            }

            ast::Stmt::ExprStmt(expr) => hir::Stmt::ExprStmt(self.lower_expr(expr)),
            ast::Stmt::Block(block) => hir::Stmt::Block(self.lower_block(block)),

            ast::Stmt::If { condition, then_branch, else_branch } => hir::Stmt::If {
                condition: self.lower_expr(condition),
                then_branch: Box::new(self.lower_stmt(then_branch)),
                else_branch: else_branch.as_deref().map(|branch| Box::new(self.lower_stmt(branch)))
            },

            ast::Stmt::For { init, condition, step, body } => {
                self.scopes.push(HashMap::new());

                let mut stmts: Vec<hir::Stmt> = Vec::new();

                //init
                if let Some(init) = init {
                    stmts.push(self.lower_stmt(init));
                }

                // cond
                let condition = match condition {
                    Some(condition) => self.lower_expr(condition),
                    None => hir::Expr::Bool(true),
                };

                // body
                let mut while_stmts = vec![
                    hir::Stmt::Block(self.lower_block(body))
                ];

                // step
                if let Some(step) = step {
                    while_stmts.push(self.lower_stmt(step));
                }

                // while
                stmts.push(hir::Stmt::While {
                    condition,
                    body: Box::new(hir::Stmt::Block(hir::Block { stmts: while_stmts }))
                });

                self.scopes.pop();

                return hir::Stmt::Block(hir::Block {stmts})
            },

            ast::Stmt::While { condition, body } => hir::Stmt::While {
                condition: self.lower_expr(condition),
                body: Box::new(self.lower_stmt(body))
            },

            ast::Stmt::Assign { target, value } => hir::Stmt::Assign {
                target: self.lower_expr(target),
                value:  self.lower_expr(value)
            },

            ast::Stmt::Return(expr) => {
                hir::Stmt::Return(expr.as_ref().map(|expr| self.lower_expr(expr)))
            }
        }
    }

    pub fn lower_expr(&mut self, expr: &ast::Expr) -> hir::Expr {
        match expr {
            ast::Expr::Number(number) => hir::Expr::Number(number.clone()),

            ast::Expr::Identifier(name) => {
                let symbol_id = self.resolve(name)
                    .unwrap_or_else(|| panic!("Unresolved identifier: {name}"));
                return hir::Expr::Identifier {
                    name: name.clone(),
                    symbol_id
                }
            },

            ast::Expr::String(value) => hir::Expr::String(value.clone()),
            ast::Expr::Bool(value) => hir::Expr::Bool(*value),

            ast::Expr::Binary { left, op, right } => hir::Expr::Binary {
                left: Box::new(self.lower_expr(left)),
                op: self.lower_binary_op(op),
                right: Box::new(self.lower_expr(right))
            },

            ast::Expr::Unary { op, expr } => hir::Expr::Unary {
                op: self.lower_unary_op(op),
                expr: Box::new(self.lower_expr(expr))
            },

            ast::Expr::Index { object, index } => hir::Expr::Index {
                object: Box::new(self.lower_expr(object)),
                index: Box::new(self.lower_expr(index))
            },

            ast::Expr::Call { callee, args } => hir::Expr::Call {
                callee: Box::new(self.lower_expr(callee)),
                args: args.iter().map(|arg| self.lower_expr(arg)).collect()
            },

            ast::Expr::Member { object, field } => hir::Expr::Member {
                object: Box::new(self.lower_expr(object)),
                field: field.clone()
            }
        }
    }

    pub fn lower_binary_op(&mut self, op: &ast::BinaryOp) -> hir::BinaryOp {
        match op {
            ast::BinaryOp::Plus => hir::BinaryOp::Plus,
            ast::BinaryOp::Minus => hir::BinaryOp::Minus,
            ast::BinaryOp::Star => hir::BinaryOp::Star,
            ast::BinaryOp::Slash => hir::BinaryOp::Slash,
            ast::BinaryOp::EqualEqual => hir::BinaryOp::EqualEqual,
            ast::BinaryOp::BangEqual => hir::BinaryOp::BangEqual,
            ast::BinaryOp::Greater => hir::BinaryOp::Greater,
            ast::BinaryOp::GreaterEqual => hir::BinaryOp::GreaterEqual,
            ast::BinaryOp::Less => hir::BinaryOp::Less,
            ast::BinaryOp::LessEqual => hir::BinaryOp::LessEqual,
            ast::BinaryOp::And => hir::BinaryOp::And,
            ast::BinaryOp::Or => hir::BinaryOp::Or
        }
    }

    pub fn lower_unary_op(&mut self, op: &ast::UnaryOp) -> hir::UnaryOp {
        match op {
            ast::UnaryOp::Minus => hir::UnaryOp::Minus,
            ast::UnaryOp::Bang => hir::UnaryOp::Bang
        }
    }

    pub fn lower_type_annotation(&mut self, annotation: &ast::TypeAnnotation) -> hir::TypeAnnotation {
        match annotation {
            ast::TypeAnnotation::Int => hir::TypeAnnotation::Int,
            ast::TypeAnnotation::Bool => hir::TypeAnnotation::Bool,
            ast::TypeAnnotation::String => hir::TypeAnnotation::String,
            ast::TypeAnnotation::Unit => hir::TypeAnnotation::Unit
        }
    }

    fn declare(&mut self, name: &str, kind: hir::SymbolKind) -> hir::SymbolId {
        let id = self.next_id();
        self.symbols.push(hir::Symbol {
            name: name.to_string(),
            kind
        });
        self.scopes.last_mut().expect("!no active scope").insert(name.to_string(), id);
        return id;
    }

    fn next_id(&mut self) -> hir::SymbolId {
        let id = hir::SymbolId(self.next_symbol_id);
        self.next_symbol_id += 1;
        return id;
    }

    fn resolve(&self, name: &str) -> Option<hir::SymbolId> {
        self.scopes.iter().rev().find_map(|scope| scope.get(name).copied())
    }
}