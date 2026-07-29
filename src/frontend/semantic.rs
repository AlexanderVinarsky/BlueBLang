use super::ast::{Block, Expr, Function, Item, Program, Stmt};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
}

pub struct SemanticAnalyzer {
    functions: HashMap<String, usize>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    fn collect_function_signatures(&mut self, program: &Program) -> Result<(), SemanticError> {
        for item in &program.items {
            match item {
                Item::Function(function) => {
                    self.register_function(function)?;
                }
            }
        }
        Ok(())
    }

    // analyze_program
    //  -> analyze_item
    //      -> analyze_function
    //          -> analyze_block
    //              -> analyze_stmt
    //                  -> analyze_expr

    pub fn analyze_program(&mut self, program: &Program) -> Result<(), SemanticError> {
        self.collect_function_signatures(program)?;
        for item in &program.items {
            self.analyze_item(item)?;
        }
        Ok(())
    }

    fn analyze_item(&mut self, item: &Item) -> Result<(), SemanticError> {
        match item {
            Item::Function(function) => {
                self.analyze_function(function)?;
                Ok(())
            }
        }
    }

    fn analyze_function(&mut self, function: &Function) -> Result<(), SemanticError> {
        self.check_duplicate_params(function)?;
        self.analyze_block(&function.body)?;
        Ok(())
    }

    fn analyze_block(&mut self, block: &Block) -> Result<(), SemanticError> {
        for stmt in &block.stmts {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        match stmt {
            Stmt::Let { value, .. } => {
                self.analyze_expr(value)?;
                Ok(())
            }

            Stmt::ExprStmt(expr) => {
                self.analyze_expr(expr)?;
                Ok(())
            }

            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.analyze_expr(expr)?;
                }
                Ok(())
            }

            Stmt::Assign { target, value } => {
                self.analyze_expr(target)?;
                self.analyze_expr(value)?;
                Ok(())
            }

            Stmt::Block(block) => {
                self.analyze_block(block)?;
                Ok(())
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.analyze_expr(condition)?;
                self.analyze_stmt(then_branch.as_ref())?;

                if let Some(else_branch) = else_branch {
                    self.analyze_stmt(else_branch.as_ref())?;
                }
                Ok(())
            }

            Stmt::While { condition, body } => {
                self.analyze_expr(condition)?;
                self.analyze_stmt(body.as_ref())?;

                Ok(())
            }
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Result<(), SemanticError> {
        match expr {
            Expr::Call { callee, args } => {
                self.analyze_expr(callee)?;
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                if let Expr::Identifier(name) = callee.as_ref() {
                    self.check_function_call(name, args.len())?;
                }
                Ok(())
            }

            Expr::Binary { left, right, .. } => {
                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
                Ok(())
            }

            Expr::Unary { expr, .. } => {
                self.analyze_expr(expr)?;
                Ok(())
            }

            Expr::Member { object, .. } => {
                self.analyze_expr(object)?;
                Ok(())
            }

            Expr::Index { object, index } => {
                self.analyze_expr(object)?;
                self.analyze_expr(index)?;
                Ok(())
            }

            Expr::Identifier(_) => Ok(()),

            _ => Ok(()),
        }
    }

    fn check_function_call(
        &self,
        name: &str,
        actual_arg_count: usize,
    ) -> Result<(), SemanticError> {
        let expected_arg_count = match self.functions.get(name) {
            Some(count) => *count,
            None => {
                return Err(SemanticError {
                    message: format!("Unknown function: {}", name),
                });
            }
        };
        if expected_arg_count != actual_arg_count {
            return Err(SemanticError {
                message: format!(
                    "Wrong number of arguments for function `{}`. Expected {}, got {}",
                    name, expected_arg_count, actual_arg_count
                ),
            });
        }
        Ok(())
    }

    fn register_function(&mut self, function: &Function) -> Result<(), SemanticError> {
        let old_value = self
            .functions
            .insert(function.name.clone(), function.params.len());
        if old_value.is_some() {
            return Err(SemanticError {
                message: format!("Duplicate function name: {}", function.name),
            });
        }
        Ok(())
    }

    fn check_duplicate_params(&self, function: &Function) -> Result<(), SemanticError> {
        let mut params = HashSet::new();
        for param in &function.params {
            if params.contains(&param.name) {
                return Err(SemanticError {
                    message: format!(
                        "Duplicate parameter name `{}` in function `{}`",
                        param.name, function.name
                    ),
                });
            }
            params.insert(param.name.clone());
        }
        Ok(())
    }
}

pub fn analyze_program(program: &Program) -> Result<(), SemanticError> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(program)
}
