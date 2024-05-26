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
}

impl Token {
    pub fn precedence(&self) -> Precedence {
        match self {
            Token::Plus => Precedence::Additive,
            _ => Precedence::Lowest,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            Self::Illegal(val) => write!(f, "[Illegal Token] {}", val),
            Self::Eof => write!(f, "EOF"),
            Self::Int(int) => write!(f, "{}", int),
            Self::String(str) => write!(f, "{}", str),
            Self::Identifier(ident) => write!(f, "{}", ident),
            Self::Plus => write!(f, "+"),
        }
    }
}
