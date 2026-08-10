use super::ast::{BinaryOp, Block, Expr, Function, Item, Program, Stmt, TypeAnnotation, UnaryOp};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
}

pub struct FunctionInfo {
    param_types: Vec<Type>,
    return_type: Type,
}

pub struct SemanticAnalyzer {
    functions: HashMap<String, FunctionInfo>,
    scopes: Vec<HashMap<String, Type>>,
    current_return_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Int,
    Bool,
    String,
    Unit,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            scopes: Vec::new(),
            current_return_type: None
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
        self.check_main()?;
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
        let return_type = self.type_annotation_to_type(&function.return_type);
        let previous_return_type = self.current_return_type.replace(return_type);
        self.scopes.push(HashMap::new());
        for param in &function.params {
            let ty = self.type_annotation_to_type(&param.type_annotation);
            self.declare_variable(param.name.as_str(), ty)?;
        }
        self.analyze_block(&function.body)?;
        self.scopes.pop();
        self.current_return_type = previous_return_type;
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
            Stmt::Let { value, name } => {
                let value_type = self.analyze_expr(value)?;
                self.declare_variable(name.as_str(), value_type)?;
                Ok(())
            }

            Stmt::ExprStmt(expr) => {
                self.analyze_expr(expr)?;
                Ok(())
            }

            Stmt::Return(expr) => {
                let expected = self.current_return_type.clone().expect("return outside function");

                match expr {
                    Some(expr) => {
                        let actual = self.analyze_expr(expr)?;
                        self.expect_type(&actual, &expected, "return value")?;
                        Ok(())
                    }

                    None => {
                        self.expect_type(&Type::Unit, &expected, "return value")?;
                        Ok(())
                    }
                }
            }

            Stmt::Assign { target, value } => {
                let target_type = self.analyze_assignment_target(target)?;
                let value_type = self.analyze_expr(value)?;

                self.expect_same_type(&target_type, &value_type, "assignment")?;

                Ok(())
            }

            Stmt::Block(block) => {
                self.scopes.push(HashMap::new());
                self.analyze_block(block)?;
                self.scopes.pop();
                Ok(())
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_type = self.analyze_expr(condition)?;
                self.expect_type(&condition_type, &Type::Bool, "if condition")?;
                self.analyze_stmt(then_branch.as_ref())?;
                if let Some(else_branch) = else_branch {
                    self.analyze_stmt(else_branch.as_ref())?;
                }

                Ok(())
            }

            Stmt::While { condition, body } => {
                let condition_type = self.analyze_expr(condition)?;
                self.expect_type(&condition_type, &Type::Bool, "while condition")?;
                self.analyze_stmt(body.as_ref())?;
                Ok(())
            }
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Result<Type, SemanticError> {
        match expr {
            Expr::Call { callee, args } => {
                let name = self.check_call_target(callee.as_ref())?;
                self.check_function_call(name, args.len())?;
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                Ok(Type::Unit)
            }

            Expr::Binary { left, op, right} => {
                let left_type = self.analyze_expr(left)?;
                let right_type = self.analyze_expr(right)?;

                match op {
                    BinaryOp::Plus | BinaryOp::Minus | BinaryOp::Star | BinaryOp::Slash => {
                        self.expect_type(&left_type, &Type::Int, "left operand")?;
                        self.expect_type(&right_type, &Type::Int, "right operand")?;
                        Ok(Type::Int)
                    }

                    BinaryOp::EqualEqual | BinaryOp::BangEqual => {
                        self.expect_same_type(&left_type, &right_type, "equality operands")?;
                        Ok(Type::Bool)
                    }

                    BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => {
                        self.expect_type(&left_type, &Type::Int, "left operand")?;
                        self.expect_type(&right_type, &Type::Int, "right operand")?;
                        Ok(Type::Bool)
                    }

                    BinaryOp::And | BinaryOp::Or => {
                        self.expect_type(&left_type, &Type::Bool, "left operand")?;
                        self.expect_type(&right_type, &Type::Bool, "right operand")?;
                        Ok(Type::Bool)
                    }
                }
            }

            Expr::Unary {op, expr} => {
                let expr_type = self.analyze_expr(expr)?;
                match op {
                    UnaryOp::Bang => {
                        self.expect_type(&expr_type, &Type::Bool, "unary ! operand")?;
                        Ok(Type::Bool)
                    }

                    UnaryOp::Minus => {
                        self.expect_type(&expr_type, &Type::Int, "unary - operand")?;
                        Ok(Type::Int)
                    }
                }
            }

            Expr::Member { object, .. } => {
                self.analyze_expr(object)?;
                Ok(Type::Int)                                           // !!!
            }

            Expr::Index { object, index } => {
                self.analyze_expr(object)?;
                let index_type = self.analyze_expr(index)?;
                self.expect_type(&index_type, &Type::Int, "index expression")?;
                Ok(Type::Int)                                           // !!!
            }

            Expr::Identifier(name) => self.resolve_variable(name),

            Expr::Number(_) => Ok(Type::Int),

            Expr::Bool(_) => Ok(Type::Bool),

            _ => Ok(Type::Unit)
        }
    }

    fn check_function_call(&self, name: &str, actual_arg_count: usize) -> Result<Type, SemanticError> {
        let info = match self.functions.get(name) {
            Some(info) => info,
            None => {
                return Err(SemanticError {
                    message: format!("Unknown function: {}", name),
                });
            }
        };
        if info.param_types.len() != actual_arg_count {
            return Err(SemanticError {
                message: format!(
                    "Wrong number of arguments for function `{}`. Expected {}, got {}",
                    name, info.param_types.len(), actual_arg_count
                ),
            });
        }
        Ok(info.return_type.clone())
    }

    fn register_function(&mut self, function: &Function) -> Result<(), SemanticError> {
        if self.functions.contains_key(&function.name) {
            return Err(SemanticError {
                message: format!("Duplicate function name: {}", function.name),
            });
        }
        let mut param_types = Vec::new();
        for param in &function.params {
            param_types.push(self.type_annotation_to_type(&param.type_annotation));
        }
        let info = FunctionInfo {
            param_types,
            return_type: self.type_annotation_to_type(&function.return_type),
        };
        self.functions.insert(function.name.clone(), info);
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

    fn check_call_target<'a>(&self, callee: &'a Expr) -> Result<&'a str, SemanticError> {
        match callee {
            Expr::Identifier(name) => Ok(name.as_str()),
            _ => Err(SemanticError {
                message: "Call target is invalid".into(),
            }),
        }
    }

    fn check_main(&self) -> Result<(), SemanticError> {
        match self.functions.get("main") {
            None => Err(SemanticError {
                message: "Main function was not found".into(),
            }),
            Some(info) => {
                if info.param_types.len() != 0 {
                    return Err(SemanticError {
                        message: format!(
                            "Main function must have 0 parameters, got {}",
                            info.param_types.len()
                        ),
                    });
                }
                Ok(())
            }
        }
    }

    fn declare_variable(&mut self, name: &str, ty: Type) -> Result<(), SemanticError> {
        let scope = self.scopes.last_mut().expect("No active scope");
        if scope.contains_key(name) {
            return Err(SemanticError {
                message: format!("Duplicate variable name: {}", name),
            });
        }
        scope.insert(name.to_string(), ty);
        Ok(())
    }

    fn resolve_variable(&self, name: &str) -> Result<Type, SemanticError> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Ok(ty.clone());
            }
        }
        Err(SemanticError {
            message: format!("Unknown variable: {}", name),
        })
    }

    fn analyze_assignment_target(&mut self, target: &Expr) -> Result<Type, SemanticError> {
        match target {
            Expr::Identifier(name) => self.resolve_variable(name),
            Expr::Member { object, .. } => {
                self.analyze_expr(object)?;
                Ok(Type::Int)                                                               // !!!
            }
            Expr::Index { object, index } => {
                self.analyze_expr(object)?;
                self.analyze_expr(index)?;
                Ok(Type::Int)                                                               // !!!
            }
            _ => Err(SemanticError {
                message: "Invalid assignment target".into(),
            }),
        }
    }

    fn expect_type(&self, actual: &Type, expected: &Type, context: &str) -> Result<(), SemanticError> {
        if actual != expected {
            return Err(SemanticError {
                message: format!(
                    "Type mismatch in {}: expected {:?}, got {:?}",
                    context, expected, actual
                ),
            });
        }
        Ok(())
    }

    fn expect_same_type(&self, left: &Type, right: &Type, context: &str) -> Result<(), SemanticError> {
        if left != right {
            return Err(SemanticError {
                message: format!(
                    "Type mismatch in {}: left is {:?}, right is {:?}",
                    context, left, right
                ),
            });
        }

        Ok(())
    }

    fn type_annotation_to_type(&self, ty: &TypeAnnotation) -> Type {
        match ty {
            TypeAnnotation::Int => Type::Int,
            TypeAnnotation::Bool => Type::Bool,
            TypeAnnotation::String => Type::String,
            TypeAnnotation::Unit => Type::Unit,
        }
    }
}

pub fn analyze_program(program: &Program) -> Result<(), SemanticError> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(program)
}
