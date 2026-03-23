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