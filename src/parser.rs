use crate::ast::{Expr, BinaryOp, Stmt};
use crate::token::{Token, TokenKind};





#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {


    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    // private

    fn current(&self)-> &Token {
        &self.tokens[self.pos]
    }

    fn token_to_binary_op(kind: TokenKind) -> BinaryOp {
        match kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::EqualEqual => BinaryOp::EqualEqual,
            _ => panic!("expected binary operator"),
        }
    }


    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }


    fn check_kind(&self, kind: TokenKind)-> bool {
        !self.is_eof() && self.current().kind == kind
    }


    fn is_eof(&self)-> bool {
        self.current().kind == TokenKind::Eof
    }


    fn expect_kind(&mut self, kind: TokenKind) {
        if self.check_kind(kind.clone()) {
            self.advance();
        } else {
            panic!("expected token");
        }
    }


    fn expect_identifier(&mut self) -> String {
        if self.check_kind(TokenKind::Identifier) {
            let name = self.current().text.clone();
            self.advance();
            return name
        } else {
            panic!("expected identifier");
        }
    }






    // public

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts= Vec::new();

        while !self.is_eof() {
            stmts.push(self.parse_statement());
        }
        return stmts;
    }


    fn parse_statement(&mut self) -> Stmt {
        if self.check_kind(TokenKind::Let) {
            self.parse_let_stmt()
        } else {
            self.parse_expr_stmt()
        }
    }


    fn parse_let_stmt(&mut self) -> Stmt {
        self.expect_kind(TokenKind::Let);

        let name = self.expect_identifier();
        self.expect_kind(TokenKind::Equal);

        let value = self.parse_expr();

        self.expect_kind(TokenKind::Semicolon);

        return Stmt::Let {name, value};
    }


    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();
        self.expect_kind(TokenKind::Semicolon);
        return Stmt::ExprStmt(expr)
    }









    fn parse_expr(&mut self) -> Expr {
        self.parse_equality()                       // [!!!]
    }



    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_additive();

        while self.check_kind(TokenKind::EqualEqual) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_additive();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return left;
    }


    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();

        while self.check_kind(TokenKind::Plus) || self.check_kind(TokenKind::Minus) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_multiplicative();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return left;
    }


    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_primary();

        while self.check_kind(TokenKind::Star) || self.check_kind(TokenKind::Slash) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_primary();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return left;
    }






    fn parse_primary(&mut self) -> Expr {
        if self.check_kind(TokenKind::Number) {
            let text = self.current().text.clone();
            self.advance();
            return Expr::Number(text);
        }


        if self.check_kind(TokenKind::String) {
            let text = self.current().text.clone();
            self.advance();
            return Expr::String(text);
        }


        if self.check_kind(TokenKind::Identifier) {
            let text = self.current().text.clone();
            self.advance();
            return Expr::Identifier(text);
        }


        if self.check_kind(TokenKind::LParen) {
            self.advance();
            let expr = self.parse_expr();
            self.expect_kind(TokenKind::RParen);
            return expr;
        }

        panic!("expected primary expression");
    }




    

}
