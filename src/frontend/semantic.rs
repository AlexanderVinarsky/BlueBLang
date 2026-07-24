use super::ast::{Function, Item, Program};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticError {
    pub message: String,
}

pub struct SemanticAnalyzer {
    functions: HashSet<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            functions: HashSet::new(),
        }
    }

    pub fn analyze_program(&mut self, program: &Program) -> Result<(), SemanticError> {
        for item in &program.items {
            self.analyze_item(item)?;
        }

        Ok(())
    }

    fn analyze_item(&mut self, item: &Item) -> Result<(), SemanticError> {
        match item {
            Item::Function(function) => {
                if self.functions.contains(&function.name) {
                    return Err(SemanticError {
                        message: format!("Duplicate function name: {}", function.name),
                    });
                }
                self.functions.insert(function.name.clone());
                Ok(())
            }
        }
    }

    fn register_function(&mut self, function: &Function) -> Result<(), SemanticError> {
        if self.functions.contains(&function.name) {
            return Err(SemanticError {
                message: format!("duplicate function name: {}", function.name),
            });
        }
        self.functions.insert(function.name.clone());
        Ok(())
    }

    fn analyze_function(&mut self, function: &Function) -> Result<(), SemanticError> {
        self.check_duplicate_params(function)?;
        Ok(())
    }

    fn check_duplicate_params(&self, function: &Function) -> Result<(), SemanticError> {
        let mut params = HashSet::new();
        for param in &function.params {
            if params.contains(&param.name) {
                return Err(SemanticError {
                    message: format!(
                        "duplicate parameter name `{}` in function `{}`",
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
