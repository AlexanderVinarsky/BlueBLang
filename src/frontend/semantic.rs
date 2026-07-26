use super::ast::{Block, Function, Item, Program};
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

    fn register_function(&mut self, function: &Function) -> Result<(), SemanticError> {
        let old_value = self
            .functions
            .insert(function.name.clone(), function.params.len());
        if old_value.is_some() {
            return Err(SemanticError {
                message: format!("duplicate function name: {}", function.name),
            });
        }
        Ok(())
    }

    fn analyze_function(&mut self, function: &Function) -> Result<(), SemanticError> {
        self.check_duplicate_params(function)?;
        self.analyze_block(&function.body)?;
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

    fn analyze_block(&mut self, block: &Block) -> Result<(), SemanticError> {
        for stmt in &block.stmts {
            //self.analyze_stmt(stmt)?;
        }
        Ok(())
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
}

pub fn analyze_program(program: &Program) -> Result<(), SemanticError> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(program)
}
