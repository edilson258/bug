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

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Infix(Box<Expression>, Infix, Box<Expression>),
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
