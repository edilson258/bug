use core::fmt;

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
pub struct FunctionCall {
    name: String,
    args: Vec<Expression>,
}

impl FunctionCall {
    pub fn make(name: String, args: Vec<Expression>) -> Self {
        Self { name, args }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Infix(Box<Expression>, Infix, Box<Expression>),
    FunctionCall(FunctionCall),
}

pub type BlockStatment = Vec<Statment>;

#[derive(Debug)]
pub struct FunctionDeclaration {
    name: String,
    body: BlockStatment,
}

impl FunctionDeclaration {
    pub fn make(name: String, body: BlockStatment) -> Self {
        Self { name, body }
    }
}

#[derive(Debug)]
pub enum Statment {
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    Additive = 1,
    Call = 2,
}
