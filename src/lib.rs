pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

use ast::Program;
use lexer::Lexer;
use parser::Parser;

pub type FrontendResult<T>= Result<T, FrontendError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub message:String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message:String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrontendError {
    Lex(LexError),
    Parse(ParseError),
}

impl From<LexError> for FrontendError {
    fn from(err:LexError) -> Self {
        Self::Lex(err)
    }
}

impl From<ParseError> for FrontendError {
    fn from(err:ParseError) -> Self {
        Self::Parse(err)
    }
}

pub fn parse_program(src:&str) -> FrontendResult<Program> {
    let tokens= Lexer::new(src).tokenize()?;
    let mut parser= Parser::new(tokens);
    Ok(parser.parse_program()?)
}

