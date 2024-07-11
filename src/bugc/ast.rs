use core::fmt;

use bug::Type;

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Option<String>),
    If(BlockStatement),
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}

pub type BlockStatement = Vec<Statement>;
pub type AST = BlockStatement;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub type_: Type,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnParam {
    pub name: String,
    pub type_: Type,
}

pub type FnParams = Vec<FnParam>;

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub params: FnParams,
    pub return_type: Type,
    pub body: BlockStatement,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    FunctionCall(String),
    BinaryOp(BinaryOp),
    Return(Option<Type>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
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
