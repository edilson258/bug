use core::fmt;

use bug::Type;

#[derive(Debug)]
pub enum Statement {
    If(BlockStatement),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
}

pub type BlockStatement = Vec<Statement>;
pub type AST = BlockStatement;

#[derive(Debug)]
pub struct FnParam {
    pub name: String,
    pub type_: Type,
}

pub type FnParams = Vec<FnParam>;

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub params: FnParams,
    pub return_type: Type,
    pub body: BlockStatement,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    FunctionCall(String),
    BinaryOp(BinaryOp),
    Return(Option<Type>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    String(String),
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus(Option<Type>),
    GratherThan(Option<Type>),
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus(_) => write!(f, "+"),
            Self::GratherThan(_) => write!(f, ">"),
        }
    }
}
