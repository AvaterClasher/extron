use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Eof,
    Illegal,
    Comment,

    // Literals
    Ident(String),
    Number(f64),
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
    Set,
    Fn,
    If,
    Else,
    Return,
    Import,
    Loop,
    Break,
    Continue,
    Typeof
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
