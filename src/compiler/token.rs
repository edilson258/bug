use core::fmt;

use super::ast::Precedence;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(char),
    Eof,

    Int(i32),
    String(String),
    Identifier(String),

    Plus,
    Minus,

    Dot,
    Arrow,
    Lparen,
    Rparen,
    Semicolon,
}

impl Token {
    pub fn precedence(&self) -> Precedence {
        match self {
            Token::Lparen => Precedence::Call,
            Token::Plus => Precedence::Additive,
            _ => Precedence::Lowest,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Illegal(val) => write!(f, "[Illegal Token] {}", val),
            Self::Eof => write!(f, "EOF"),
            Self::Int(int) => write!(f, "{}", int),
            Self::String(str) => write!(f, "{}", str),
            Self::Identifier(ident) => write!(f, "{}", ident),
            Self::Plus => write!(f, "+"),
            Self::Dot => write!(f, "."),
            Self::Arrow => write!(f, "->"),
            Self::Lparen => write!(f, "("),
            Self::Rparen => write!(f, ")"),
            Self::Semicolon => write!(f, ";"),
            Self::Minus => write!(f, "-"),
        }
    }
}
