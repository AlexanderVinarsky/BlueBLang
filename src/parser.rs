use crate::ast::*;
use crate::token::{Token, TokenKind};
use crate::ParseError;




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
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
    }


    fn check_kind(&self, kind:TokenKind)->bool {
        self.current().kind == kind
    }

    fn peek_kind(&self)->&TokenKind {
        &self.tokens[self.pos].kind
    }



    fn next_kind(&self) -> &TokenKind {
        if self.pos + 1 < self.tokens.len(){
            return &self.tokens[self.pos + 1].kind;
        }
        else {
            panic!("expected token, got out of bounds");
        }    
    }

    fn check_next_kind(&self, kind:TokenKind) -> bool {
        return self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].kind == kind;
    }

    fn is_assign_stmt(&self) -> bool {
        return self.check_kind(TokenKind::Identifier) && self.check_next_kind(TokenKind::Equal);
    }



    fn is_eof(&self)-> bool {
        self.current().kind == TokenKind::Eof
    }


    fn expect_kind(&mut self, kind:TokenKind) -> Result<(), ParseError> {
        if self.check_kind(kind.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {message: format!("expected token: {:?}, found: {:?}", kind, self.current().kind)})
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        if self.check_kind(TokenKind::Identifier) {
            let name= self.current().text.clone();
            self.advance();
            Ok(name)
        } else {
            Err(ParseError {message: format!("expected identifier, found: {:?}", self.current().text)})
        }
    }

    





    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items= Vec::new();

        while !self.is_eof() {
            items.push(self.parse_item()?);
        }
        return Ok(Program{items});
    }



    fn parse_item(&mut self) -> Result<Item, ParseError> {
        match self.current().kind {
            TokenKind::Fn => Ok(Item::Function(self.parse_function()?)),
            _ => Err(ParseError {
                message:"function expected".into(),
            }),
        }
    }


    fn parse_function(&mut self) -> Result<Function, ParseError> {

        self.expect_kind(TokenKind::Fn)?;

        let name= self.expect_identifier()?;

        self.expect_kind(TokenKind::LParen)?;

        let params= self.parse_params()?;

        self.expect_kind(TokenKind::RParen)?;

        let body= self.parse_block()?;

        Ok(Function { name, params, body })
    }



    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params= Vec::new();

        if self.check_kind(TokenKind::RParen) {
            return Ok(params);
        }

        params.push(Param{name: self.expect_identifier()?});

        while self.check_kind(TokenKind::Comma) {
            self.advance();
            params.push(Param{name: self.expect_identifier()?});
        }

        return Ok(params);
    }



    fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args= Vec::new();

        if self.check_kind(TokenKind::RParen) {
            return Ok(args);
        }

        args.push(self.parse_expr()?);

        while self.check_kind(TokenKind::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
        }

        return Ok(args);
    }



    fn parse_block(&mut self)->Result<Block, ParseError> {
        self.expect_kind(TokenKind::LBrace)?;

        let mut stmts= Vec::new();

        while !self.check_kind(TokenKind::RBrace) && !self.check_kind(TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }

        self.expect_kind(TokenKind::RBrace)?;

        return Ok(Block{stmts});
    }




    fn parse_stmt(&mut self)->Result<Stmt, ParseError> {
        match self.peek_kind() {
            TokenKind::Let          => self.parse_let_stmt(),
            TokenKind::If           => self.parse_if_stmt(),
            TokenKind::While        => self.parse_while_stmt(),
            TokenKind::Ret          => self.parse_return_stmt(),
            TokenKind::LBrace       => Ok(Stmt::Block(self.parse_block()?)),
            TokenKind::Identifier if self.is_assign_stmt() => self.parse_assign(),
            _ => self.parse_expr_stmt()
        }
    }


    fn parse_let_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect_kind(TokenKind::Let)?;

        let name = self.expect_identifier()?;
        self.expect_kind(TokenKind::Equal)?;

        let value = self.parse_expr()?;

        self.expect_kind(TokenKind::Semicolon)?;

        return Ok(Stmt::Let {name, value});
    }


    fn parse_if_stmt(&mut self)->Result<Stmt, ParseError> {
        self.expect_kind(TokenKind::If)?;

        let condition: Expr = self.parse_expr()?;
        let then_branch: Box<Stmt> = Box::new(self.parse_stmt()?);

        let else_branch=
            if self.check_kind(TokenKind::Else) {
                self.expect_kind(TokenKind::Else)?;
                Some(Box::new(self.parse_stmt()?))
            } else {
                None
            };

        return Ok(Stmt::If {condition, then_branch, else_branch})
    }



    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect_kind(TokenKind::While)?;

        let condition: Expr = self.parse_expr()?;
        let body: Box<Stmt> = Box::new(self.parse_stmt()?);

        return Ok(Stmt::While {condition, body})
    }



    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect_kind(TokenKind::Ret)?;
        
        if self.check_kind(TokenKind::Semicolon) {
            self.expect_kind(TokenKind::Semicolon)?;
            return Ok(Stmt::Return(None));
        }

        let value = self.parse_expr()?;
        self.expect_kind(TokenKind::Semicolon)?;

        return Ok(Stmt::Return(Some(value)))
    }

    fn parse_assign(&mut self) -> Result<Stmt, ParseError> {
        let name= self.expect_identifier()?;
        self.expect_kind(TokenKind::Equal)?;
        let value= self.parse_expr()?;
        self.expect_kind(TokenKind::Semicolon)?;
        
        return Ok(Stmt::Assign {name, value});
}



    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expr()?;
        self.expect_kind(TokenKind::Semicolon)?;
        return Ok(Stmt::ExprStmt(expr))
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
                             -> parse_call()        +
                                 -> parse_primary() +
*/



    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()                      
    }



    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;

        while self.check_kind(TokenKind::Or) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_and()?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        return Ok(left);
    }



    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_equality()?;

        while self.check_kind(TokenKind::And) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_equality()?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        return Ok(left);
    }



    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;

        while self.check_kind(TokenKind::EqualEqual) || self.check_kind(TokenKind::BangEqual) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_comparison()?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return Ok(left);
    }


    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;

        while self.check_kind(TokenKind::Greater) || self.check_kind(TokenKind::GreaterEqual)
           || self.check_kind(TokenKind::Less)    || self.check_kind(TokenKind::LessEqual)
        {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_additive()?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }



    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;

        while self.check_kind(TokenKind::Plus) || self.check_kind(TokenKind::Minus) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_multiplicative()?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return Ok(left);
    }



    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;

        while self.check_kind(TokenKind::Star) || self.check_kind(TokenKind::Slash) {
            let op = Self::token_to_binary_op(self.current().kind.clone());
            self.advance();

            let right = self.parse_unary()?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right)
            };
        }

        return Ok(left);
    }


    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.check_kind(TokenKind::Minus) || self.check_kind(TokenKind::Bang) {
            let op = Self::token_to_unary_op(self.current().kind.clone());
            self.advance();


            return Ok(Expr::Unary {
                op,
                expr: Box::new(self.parse_unary()?),
            })
        }

        self.parse_call()
    }



    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let expr= self.parse_primary()?;

        if self.check_kind(TokenKind::LParen) {
            
            match expr {
                
                Expr::Identifier(name) => {
                    self.expect_kind(TokenKind::LParen)?;
                    let args= self.parse_args()?;
                    self.expect_kind(TokenKind::RParen)?;
                    Ok(Expr::Call { name, args })
                }
                
                _ => Err(ParseError { message:"expected function name before '('".into() })
            }
        
        } else {
            Ok(expr)
        }
    }




    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if self.check_kind(TokenKind::Number) {
            let text = self.current().text.clone();
            self.advance();
            return Ok(Expr::Number(text));
        }


        if self.check_kind(TokenKind::String) {
            let text = self.current().text.clone();
            self.advance();
            return Ok(Expr::String(text));
        }

        if self.check_kind(TokenKind::True) {
            self.advance();
            return Ok(Expr::Bool(true));
        }
    
        if self.check_kind(TokenKind::False) {
            self.advance();
            return Ok(Expr::Bool(false));
        }


        if self.check_kind(TokenKind::Identifier) {
            let text = self.current().text.clone();
            self.advance();
            return Ok(Expr::Identifier(text));
        }


        if self.check_kind(TokenKind::LParen) {
            self.advance();
            let expr = self.parse_expr()?;
            self.expect_kind(TokenKind::RParen)?;
            return Ok(expr);
        }

        return Err(ParseError {message:"expected primary expression".into()});
    }

}