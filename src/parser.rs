use crate::ast::{Expr, BinaryOp, UnaryOp, Stmt};
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
            TokenKind::Plus         => BinaryOp::Plus,
            TokenKind::Minus        => BinaryOp::Minus,
            TokenKind::Star         => BinaryOp::Star,
            TokenKind::Slash        => BinaryOp::Slash,
            TokenKind::EqualEqual   => BinaryOp::EqualEqual,
            TokenKind::BangEqual    => BinaryOp::BangEqual,
            TokenKind::Greater      => BinaryOp::Greater,
            TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
            TokenKind::Less         => BinaryOp::Less,
            TokenKind::LessEqual    => BinaryOp::LessEqual,
            TokenKind::And          => BinaryOp::And,
            TokenKind::Or           => BinaryOp::Or,
            _ => panic!("expected binary operator"),
        }
    }

    fn token_to_unary_op(kind: TokenKind) -> UnaryOp {
        match kind {
            TokenKind::Minus        => UnaryOp::Minus,
            TokenKind::Bang         => UnaryOp::Bang,
            _ => panic!("expected unary operator"),
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

    fn peek_kind(&self)->&TokenKind {
        &self.tokens[self.pos].kind
    }

    fn is_eof(&self)-> bool {
        self.current().kind == TokenKind::Eof
    }


    fn expect_kind(&mut self, kind: TokenKind) {
        if self.check_kind(kind.clone()) {
            self.advance();
        } else {
            panic!("expected token: {:?}, found: {:?}", kind, self.current().kind);
        }
    }


    fn expect_identifier(&mut self) -> String {
        if self.check_kind(TokenKind::Identifier) {
            let name = self.current().text.clone();
            self.advance();
            return name
        } else {
            panic!("expected identifier, found: {:?}", self.current().text);
        }
    }








    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts= Vec::new();

        while !self.is_eof() {
            stmts.push(self.parse_stmt());
        }
        return stmts;
    }



    fn parse_stmt(&mut self)->Stmt {
        match self.peek_kind() {
            TokenKind::Let          => self.parse_let_stmt(),
            TokenKind::If           => self.parse_if_stmt(),
            TokenKind::While        => self.parse_while_stmt(),
            TokenKind::Ret          => self.parse_return_stmt(),
            TokenKind::LBrace       => self.parse_block(),
            _ => self.parse_expr_stmt()
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


    fn parse_if_stmt(&mut self)->Stmt {
        self.expect_kind(TokenKind::If);

        let condition: Expr = self.parse_expr();
        let then_branch: Box<Stmt> = Box::new(self.parse_stmt());

        let else_branch=
            if self.check_kind(TokenKind::Else) {
                self.expect_kind(TokenKind::Else);
                Some(Box::new(self.parse_stmt()))
            } else {
                None
            };

        return Stmt::If {condition, then_branch, else_branch}
    }



    fn parse_while_stmt(&mut self) -> Stmt {
        self.expect_kind(TokenKind::While);

        let condition: Expr = self.parse_expr();
        let body: Box<Stmt> = Box::new(self.parse_stmt());

        return Stmt::While {condition, body}
    }



    fn parse_return_stmt(&mut self) -> Stmt {
        self.expect_kind(TokenKind::Ret);
        
        if self.check_kind(TokenKind::Semicolon) {
            self.expect_kind(TokenKind::Semicolon);
            return Stmt::Return(None);
        }

        let value = self.parse_expr();
        self.expect_kind(TokenKind::Semicolon);

        return Stmt::Return(Some(value))
    }



    fn parse_block(&mut self) -> Stmt {
        self.expect_kind(TokenKind::LBrace);

        let mut stmts= Vec::new();

        while !self.check_kind(TokenKind::RBrace) && !self.check_kind(TokenKind::Eof) {
            stmts.push(self.parse_stmt());
        }

        self.expect_kind(TokenKind::RBrace);

        Stmt::Block(stmts)
    }



    fn parse_expr_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();
        self.expect_kind(TokenKind::Semicolon);
        return Stmt::ExprStmt(expr)
    }


//NB

/* 
parse_expr()                                        +
  -> parse_or()                                     +
      -> parse_and()                                +
          -> parse_equality()                       +
              -> parse_comparison()                 + 
                  -> parse_additive()               +
                      -> parse_multiplicative()     +
                          -> parse_unary()          +
                              -> parse_primary()    +
*/



    fn parse_expr(&mut self) -> Expr {
        self.parse_or()                      // [!!!]
    }



    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();

        while self.check_kind(TokenKind::Or) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_and();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        return left;
    }



    fn parse_and(&mut self) -> Expr {
        let mut left = self.parse_equality();

        while self.check_kind(TokenKind::And) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_equality();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        return left;
    }



    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();

        while self.check_kind(TokenKind::EqualEqual) || self.check_kind(TokenKind::BangEqual) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_comparison();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return left;
    }


    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_additive();

        while self.check_kind(TokenKind::Greater) || self.check_kind(TokenKind::GreaterEqual)
           || self.check_kind(TokenKind::Less)    || self.check_kind(TokenKind::LessEqual)
        {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_additive();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
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
        let mut left = self.parse_unary();

        while self.check_kind(TokenKind::Star) || self.check_kind(TokenKind::Slash) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_unary();

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return left;
    }


    fn parse_unary(&mut self) -> Expr {
        if self.check_kind(TokenKind::Minus) || self.check_kind(TokenKind::Bang) {
            let op = Self::token_to_unary_op(self.current().kind.clone());
            self.advance();


        let operand = self.parse_unary();

        return Expr::Unary {
            op,
            expr: Box::new(operand),
        };
    }

    self.parse_primary()
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

        if self.check_kind(TokenKind::True) {
            self.advance();
            return Expr::Bool(true);
        }
    
        if self.check_kind(TokenKind::False) {
            self.advance();
            return Expr::Bool(false);
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
