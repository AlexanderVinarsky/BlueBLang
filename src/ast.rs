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

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {name: String, value: Expr},

    ExprStmt(Expr),

    Block(Vec<Stmt>),


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