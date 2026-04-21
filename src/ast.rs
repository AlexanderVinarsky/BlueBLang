#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items:Vec<Item>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name:String,
    pub params:Vec<Param>,
    pub body:Block
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name:String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts:Vec<Stmt>,
}





#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {name: String, value: Expr},

    ExprStmt(Expr),

    Block(Block),


    If {
        condition:   Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>    
    }, 


    While {
        condition:   Expr,
        body:        Box<Stmt>
    },

    Return(Option<Expr>)
}



#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(String),
    Identifier(String),
    String(String),
    Bool(bool),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>
    }
}



#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,

    EqualEqual,
    BangEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,
}



#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Minus,
    Bang,
}