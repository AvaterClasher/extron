use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Eof,
    Illegal,

    // Literals
    Ident(String),
    Int(i32),
    String(String),
    Boolean(bool),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Percent,

    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Delimiters
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Keywords
    Let,
    Fn,
    If,
    Else,
    Return,
    Import,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
