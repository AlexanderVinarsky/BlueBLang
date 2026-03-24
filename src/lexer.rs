use core::panic;

use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}



impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            input: src.chars().collect(),
            pos: 0,
        }
    }



    pub fn has_next(&self) -> bool {
        self.pos + 1 < self.input.len()
    }

    

    pub fn next_is(&self, ch: char) -> bool {
        self.has_next() && (self.input[self.pos + 1] == ch)
    }






    pub fn tokenize(&mut self) -> Vec<Token> {
        
        
        let mut tokens = Vec::new();

        
        while self.pos < self.input.len() {
            let ch= self.input[self.pos];
            
            if ch.is_whitespace() {
                self.pos += 1;
                continue;
            }



            if ch.is_ascii_alphabetic() || ch == '_' {
                let start = self.pos;

                while self.pos < self.input.len() && (self.input[self.pos].is_ascii_alphanumeric() || self.input[self.pos] == '_')
                {
                    self.pos += 1;
                }

                let text: String = self.input[start..self.pos].iter().collect(); // !
                
                let kind = match text.as_str() {
                    "let" => TokenKind::Let,
                    _ => TokenKind::Identifier,
                };

                tokens.push(Token {kind, text});
                continue;
            }


            if ch == '"' {
                let mut res = String::new();
                let mut closed = false;
            
                while self.has_next() {
                    self.pos += 1;
                
                    if self.input[self.pos] == '"' {
                        closed = true;
                        break;
                    } else {
                        res.push(self.input[self.pos]);
                    }
                }
            
                if !closed {
                    panic!("error: unterminated string literal");
                }
            
                tokens.push(Token {
                    kind: TokenKind::String,
                    text: res,
                });
            
                self.pos += 1;
                continue;
            }


            if ch.is_ascii_digit() {
                let mut res = String::from(ch);
            
                while self.has_next() && self.input[self.pos + 1].is_ascii_digit() {
                    res.push(self.input[self.pos + 1]);
                    self.pos += 1;
                }
            
                if self.has_next() {
                    let next = self.input[self.pos + 1];
                
                    if next.is_ascii_alphabetic() || next == '_' {
                        panic!("error: invalid number literal");
                    }
                }
            
                tokens.push(Token {
                    kind: TokenKind::Number,
                    text: res,
                });
            
                self.pos += 1;
                continue;
            }






            match ch {

                '+' => {
                    tokens.push(Token {
                        kind: TokenKind::Plus,
                        text: "+".to_string(),
                    });
                    self.pos += 1;
                }

                '-' => {
                    tokens.push(Token {
                        kind: TokenKind::Minus,
                        text: "-".to_string(),
                    });
                    self.pos += 1;
                }

                '*' => {
                    tokens.push(Token {
                        kind: TokenKind::Star,
                        text: "*".to_string(),
                    });
                    self.pos += 1;
                }


                '/' => {
                    tokens.push(Token {
                        kind: TokenKind::Slash,
                        text: "*".to_string(),
                    });
                    self.pos += 1;
                }

                '=' => {     
                    if self.next_is('='){
                        tokens.push(Token {
                        kind: TokenKind::EqualEqual,
                        text: "==".to_string(),
                        });
                        self.pos += 2;  
                    }
                    else {
                        tokens.push(Token {
                            kind: TokenKind::Equal,
                            text: "=".to_string(),
                        });
                        self.pos += 1;                 
                    }
                }

                '(' => {
                    tokens.push(Token {
                        kind: TokenKind::LParen,
                        text: "(".to_string(),
                    });
                    self.pos += 1;
                }

                ')' => {
                    tokens.push(Token {
                        kind: TokenKind::RParen,
                        text: ")".to_string(),
                    });
                    self.pos += 1;
                }

                ';' => {
                    tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        text: ";".to_string(),
                    });
                    self.pos += 1;
                }

                _ => {
                    tokens.push(Token {
                        kind: TokenKind::Unknown,
                        text: ch.to_string(),
                    });
                    self.pos += 1;
                }

            }

        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            text: String::new(),
        });

        tokens
    }
}