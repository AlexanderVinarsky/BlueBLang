#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String
}