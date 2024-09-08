use crate::frontend::token::{Location, Token, TokenKind};

pub type Ast = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
  Function(StatementFunction),
  Expression(StatementExpression),
}

#[derive(Debug)]
pub enum StatementExpression {
  Call(ExpressionCall),
  Binary(ExpressionBinary),
  Literal(ExpressionLiteral),
}

#[derive(Debug)]
pub enum BinaryOperator {
  Add,
}

#[derive(Debug)]
pub struct ExpressionBinary {
  pub operator: BinaryOperator,
  pub location: Location,
}

impl ExpressionBinary {
  pub fn new(operator: BinaryOperator, location: Location) -> Self {
    Self { operator, location }
  }
}

impl std::fmt::Display for BinaryOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Add => write!(f, "+"),
    }
  }
}

#[derive(Debug)]
pub struct ExpressionCall {
  pub name_token: Token,
}

impl ExpressionCall {
  pub fn new(name_token: Token) -> Self {
    Self { name_token }
  }

  pub fn get_name(&self) -> &str {
    match self.name_token.kind {
      TokenKind::Identifier(ref name) => name,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub enum ExpressionLiteral {
  String(LiteralString),
  Number(NumberLiteral),
}

#[derive(Debug)]
pub struct LiteralString {
  pub token: Token,
}

#[derive(Debug)]
pub struct NumberLiteral {
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

  pub fn get_name(&self) -> &str {
    match self.identifier.kind {
      TokenKind::Identifier(ref str) => str,
      _ => unreachable!(),
    }
  }
}

impl LiteralString {
  pub fn new(token: Token) -> Self {
    Self { token }
  }

  pub fn get_data(&self) -> &str {
    match &self.token.kind {
      TokenKind::String(string) => string,
      _ => unreachable!(),
    }
  }
}

impl NumberLiteral {
  pub fn new(token: Token) -> Self {
    Self { token }
  }

  pub fn get_data(&self) -> &f32 {
    match &self.token.kind {
      TokenKind::Number(number) => number,
      _ => unreachable!(),
    }
  }
}
