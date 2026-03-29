#[derive(Debug)]
pub enum Expr {
    Number(String),
    Identifier(String),
    String(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    EqualEqual,
}

#[derive(Debug)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    ExprStmt(Expr),
}