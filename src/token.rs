#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Identifier,
    Number,
    String,

    Let,
    If,
    Else,
    While,
    For,
    Ret,
    True,
    False,

    Plus,
    Minus,
    Star,
    Slash,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Comma,
    Dot,
    Colon,
    Semicolon,

    Eof,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String
}