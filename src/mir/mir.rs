use crate::hir::hir::{Symbol, SymbolId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub u32);

pub struct BasicBlock {
    pub statements: Vec<Instruction>,
    pub terminator: Terminator
}

pub struct Program {
    pub functions: Vec<Function>,
    pub symbols: Vec<Symbol>
}

pub struct Function {
    pub name: String,
    pub symbol_id: SymbolId,
    pub params: Vec<LocalId>,
    pub return_type: Type,

    pub locals: Vec<LocalDecl>,
    pub blocks: Vec<BasicBlock>
}

pub enum Type {
    Int,
    Bool,
    String,
    Unit
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Place {
    Local(LocalId)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Local(LocalId),
    Constant(Constant)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    Int(i64),
    Bool(bool),
    String(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Terminator {
    Goto(BlockId),

    Branch {
        condition: Operand,
        then_block: BlockId,
        else_block: BlockId
    },

    Return(Option<Operand>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Copy {
        destination: Place,
        source: Operand
    },

    Binary {
        destination: Place,
        op: BinaryOp,
        lhs: Operand,
        rhs: Operand
    },

    Unary {
        destination: Place,
        op: UnaryOp,
        operand: Operand
    },

    Call {
        destination: Option<Place>,
        function: SymbolId,
        args: Vec<Operand>
    }
}

pub struct LocalDecl {
    pub ty: Type,
    pub symbol_id: Option<SymbolId>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Ne,

    Gt,
    Ge,
    Lt,
    Le
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not
}