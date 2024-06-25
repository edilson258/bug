use spider_vm::std::Type;

pub type AST = Vec<Statment>;

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    String(String),
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
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
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Type,
    pub body: BlockStatment,
}

impl FunctionDeclaration {
    pub fn make(name: String, return_type: Type, body: BlockStatment) -> Self {
        Self {
            name,
            return_type,
            body,
        }
    }
}

#[derive(Debug)]
pub enum Statment {
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
}
