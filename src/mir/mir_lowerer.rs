use crate::hir::hir;
use crate::mir::mir;
use std::collections::HashMap;
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

    fn finish(self) -> (Vec<mir::LocalDecl>, Vec<mir::BasicBlock>) {
        let blocks = self.blocks.into_iter().map(|block| {
                mir::BasicBlock {
                    instructions: block.instructions,
                    terminator: block.terminator.expect("!basic block has no terminator")
                }
            })
            .collect();
        (self.locals, blocks)
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
        //self.lower_block(); todo
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
}