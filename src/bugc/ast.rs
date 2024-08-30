use crate::frontend::token::Token;

pub type Ast = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
  Function(StatementFunction),
  Expression(StatementExpression),
}

#[derive(Debug)]
pub enum StatementExpression {
  Call(ExpressionCall),
  Literal(ExpressionLiteral),
}

#[derive(Debug)]
pub struct ExpressionCall {
  pub name_token: Token,
}

impl ExpressionCall {
  pub fn new(name_token: Token) -> Self {
    Self { name_token }
  }
}

#[derive(Debug)]
pub enum ExpressionLiteral {
  String(LiteralString),
  Integer(LiteralInteger),
}

#[derive(Debug)]
pub struct LiteralString {
  pub token: Token,
}

#[derive(Debug)]
pub struct LiteralInteger {
  pub token: Token,
}

#[derive(Debug)]
pub struct StatementFunction {
  pub identifier: Token,
  pub body: Vec<Statement>,
}

impl StatementFunction {
  pub fn new(identifier: Token, body: Vec<Statement>) -> Self {
    Self { identifier, body }
  }
}

impl LiteralString {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

impl LiteralInteger {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}
