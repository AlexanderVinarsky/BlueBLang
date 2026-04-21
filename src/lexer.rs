use crate::token::{Token, TokenKind};
use crate::LexError;

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






    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        
        
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
                    "let"   => TokenKind::Let,
                    "if"    => TokenKind::If,
                    "else"  => TokenKind::Else,
                    "while" => TokenKind::While,  
                    "for"   => TokenKind::For,
                    "ret"   => TokenKind::Ret, 
                    "true"  => TokenKind::True,
                    "false" => TokenKind::False,
                    "and"   => TokenKind::And,
                    "or"    => TokenKind::Or,
                    "fn"    => TokenKind::Fn,
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
                    return Err(LexError {message:"unterminated string literal".into(),});
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
                        return Err(LexError {message:"invalid number literal".into(),});
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
                        text: "/".to_string(),
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

                '>' => {     
                    if self.next_is('='){
                        tokens.push(Token {
                        kind: TokenKind::GreaterEqual,
                        text: ">=".to_string(),
                        });
                        self.pos += 2;  
                    }
                    else {
                        tokens.push(Token {
                            kind: TokenKind::Greater,
                            text: ">".to_string(),
                        });
                        self.pos += 1;                 
                    }
                }

                '<' => {     
                    if self.next_is('='){
                        tokens.push(Token {
                        kind: TokenKind::LessEqual,
                        text: "<=".to_string(),
                        });
                        self.pos += 2;  
                    }
                    else {
                        tokens.push(Token {
                            kind: TokenKind::Less,
                            text: "<".to_string(),
                        });
                        self.pos += 1;                 
                    }
                }

                '!' => {     
                    if self.next_is('='){
                        tokens.push(Token {
                        kind: TokenKind::BangEqual,
                        text: "!=".to_string(),
                        });
                        self.pos += 2;  
                    }
                    else {
                        tokens.push(Token {
                            kind: TokenKind::Bang,
                            text: "!".to_string(),
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

                '{' => {
                    tokens.push(Token {
                        kind: TokenKind::LBrace,
                        text: "{".to_string(),
                    });
                    self.pos += 1;
                }

                '}' => {
                    tokens.push(Token {
                        kind: TokenKind::RBrace,
                        text: "}".to_string(),
                    });
                    self.pos += 1;
                }

                '[' => {
                    tokens.push(Token {
                        kind: TokenKind::LBracket,
                        text: "[".to_string(),
                    });
                    self.pos += 1;
                }

                ']' => {
                    tokens.push(Token {
                        kind: TokenKind::RBracket,
                        text: "]".to_string(),
                    });
                    self.pos += 1;
                }

                '.' => {
                    tokens.push(Token {
                        kind: TokenKind::Dot,
                        text: ".".to_string(),
                    });
                    self.pos += 1;
                }

                ',' => {
                    tokens.push(Token {
                        kind: TokenKind::Comma,
                        text: ",".to_string(),
                    });
                    self.pos += 1;
                }

                ':' => {
                    tokens.push(Token {
                        kind: TokenKind::Colon,
                        text: ":".to_string(),
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
                    return Err(LexError {message:format!("unexpected character: {}", ch),});
                }

            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            text: String::new(),
        });

        

        return Ok(tokens);
    }
}