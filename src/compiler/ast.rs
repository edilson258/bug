use core::fmt;

use crate::analysis::Type;

pub type AST = Vec<Statment>;

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    String(String),
}

#[derive(Debug)]
pub enum Infix {
    Plus,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Infix(Box<Expression>, Infix, Box<Expression>),
}

impl Expression {
    pub fn ask_type(&self) -> Type {
        match self {
            Self::Literal(literal) => match literal {
                Literal::Int(_) => Type::Integer,
                Literal::String(_) => Type::String,
            },
            Self::Infix(lhs, _, _) => lhs.ask_type(),
        }
    }
}

#[derive(Debug)]
pub enum Statment {
    Expression(Expression),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    Additive = 1,
}
