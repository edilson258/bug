use spider_vm::stdlib::Type;

pub type AST = Vec<Statment>;

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

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    FunctionCall(String),
    BinaryOp(BinaryOp),
}

pub type BlockStatment = Vec<Statment>;

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
    pub body: BlockStatment,
}

impl FunctionDeclaration {
    pub fn make(name: String, params: FnParams, return_type: Type, body: BlockStatment) -> Self {
        Self {
            name,
            params,
            return_type,
            body,
        }
    }
}

#[derive(Debug)]
pub enum Statment {
    If(BlockStatment),
    Return(Option<Type>),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
}
