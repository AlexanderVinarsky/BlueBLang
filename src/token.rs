#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind{
    Identifier,
    Number,
    String,

    Let,

    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,

    LParen,
    RParen,
    Semicolon,

    Eof,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String
}