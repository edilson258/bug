use super::lexer::{Lexer, LexerError};
use super::token::{Token, TokenKind};
use crate::ast::*;
use crate::span::Span;
use bug::Type;

pub struct Parser<'a> {
  current_token: Token,
  next_token: Token,
  lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
    Self { lexer, current_token: Token::default(), next_token: Token::default() }
  }

  pub fn parse(&mut self) -> Result<Ast, ParserError> {
    self.bump()?;
    self.bump()?;
    let mut ast: Ast = vec![];
    while self.current_token.kind != TokenKind::Eof {
      ast.push(self.parse_statement()?);
    }
    Ok(ast)
  }

  fn bump(&mut self) -> Result<(), ParserError> {
    self.current_token = self.next_token.clone();
    self.next_token = match self.lexer.next_token() {
      Ok(token) => token,
      Err(error) => return Err(ParserError::from_lexer_error(error)),
    };
    Ok(())
  }

  fn bump_expect(&mut self, expected: TokenKind, message: &str) -> Result<(), ParserError> {
    if expected != self.current_token.kind {
      return Err(ParserError::new(message.to_string(), self.current_token.span.clone()));
    }
    self.bump()
  }

  fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    match self.current_token.kind {
      TokenKind::Function => Ok(Statement::Function(self.parse_statement_function()?)),
      _ => Ok(Statement::Expression(self.parse_statement_expression()?)),
    }
  }

  fn parse_statement_function(&mut self) -> Result<StatementFunction, ParserError> {
    self.bump()?;
    let identifier = self.parse_identifier()?;
    let mut signature_span = self.current_token.span.clone();
    let parameters = self.parse_function_params()?;
    signature_span.end = self.current_token.span.end;
    let return_type = self.parse_type_annotation()?;
    let body = self.parse_statement_block()?;
    Ok(StatementFunction::new(identifier, parameters, return_type, body, signature_span))
  }

  fn parse_statement_block(&mut self) -> Result<StatementBlock, ParserError> {
    let mut block = StatementBlock::new();
    block.span = self.current_token.span.clone();
    self.bump_expect(TokenKind::Arrow, "Expecting `->` to mark the start of the block")?;
    while self.current_token.kind != TokenKind::Semicolon {
      block.statements.push(self.parse_statement()?);
    }
    block.span.end = self.current_token.span.end;
    self.bump_expect(TokenKind::Semicolon, "Expecting `;` to mark the end of the block")?;
    Ok(block)
  }

  fn parse_function_params(&mut self) -> Result<Parameters, ParserError> {
    let mut parameters = Parameters::new();
    parameters.span = self.current_token.span.clone();
    self.bump_expect(TokenKind::LeftParent, "Expecting `(` after function's name")?;
    while self.current_token.kind != TokenKind::RightParent {
      let mut parameter_span = self.current_token.span.clone();
      let parameter_type = self.parse_type_annotation()?;
      let parameter_name = self.parse_identifier()?;
      parameter_span.end = parameter_name.span.end;
      parameters.parameters.push(Parameter::new(parameter_name, parameter_type, parameter_span));
      match self.current_token.kind {
        TokenKind::Comma => self.bump()?,
        TokenKind::RightParent => break,
        _ => return Err(ParserError::expect_either(")", ",", self.current_token.span.clone())),
      };
    }
    parameters.span.end = self.current_token.span.end;
    self.bump_expect(TokenKind::RightParent, "Expecting `)` after function's parameters")?;
    Ok(parameters)
  }

  fn parse_type_annotation(&mut self) -> Result<Type, ParserError> {
    let typ = match self.current_token.kind {
      TokenKind::TypeInt => Type::Integer,
      TokenKind::TypeVoid => Type::Void,
      _ => return Err(ParserError::expect_type_annotation(self.current_token.span.clone())),
    };
    self.bump()?;
    Ok(typ)
  }

  fn parse_identifier(&mut self) -> Result<Identifier, ParserError> {
    let identifier = match &self.current_token.kind {
      TokenKind::Identifier(label) => Identifier::new(self.current_token.span.clone(), label.clone()),
      _ => return Err(ParserError::expect_identifier(self.current_token.span.clone())),
    };
    self.bump()?;
    Ok(identifier)
  }

  fn parse_statement_expression(&mut self) -> Result<StatementExpression, ParserError> {
    match self.current_token.kind {
      TokenKind::At => Ok(StatementExpression::Call(self.parse_expession_call()?)),
      TokenKind::String(_) | TokenKind::Integer(_) => Ok(StatementExpression::Literal(self.parse_expession_literal()?)),
      TokenKind::Plus => Ok(StatementExpression::Binary(self.parse_expression_binary()?)),
      _ => Err(ParserError::unexpected_expression_token(self.current_token.clone())),
    }
  }

  fn parse_expression_binary(&mut self) -> Result<ExpressionBinary, ParserError> {
    let op = match self.current_token.kind {
      TokenKind::Plus => BinaryOperator::Plus,
      TokenKind::Minus => BinaryOperator::Minus,
      _ => unreachable!("Invalid binary operator {:#?}", self.current_token.kind),
    };
    let binary_expression = ExpressionBinary::new(op, self.current_token.span.clone());
    self.bump()?;
    Ok(binary_expression)
  }

  fn parse_expession_literal(&mut self) -> Result<ExpressionLiteral, ParserError> {
    match self.current_token.kind.clone() {
      TokenKind::String(inner) => Ok(ExpressionLiteral::String(self.parse_literal_string(inner)?)),
      TokenKind::Integer(inner) => Ok(ExpressionLiteral::Integer(self.parse_literal_integer(inner)?)),
      _ => unreachable!("Invalid literal {:#?}", self.current_token.kind),
    }
  }

  fn parse_literal_string(&mut self, inner: String) -> Result<LiteralString, ParserError> {
    let string_literal = LiteralString::new(self.current_token.span.clone(), inner);
    self.bump()?;
    Ok(string_literal)
  }

  fn parse_literal_integer(&mut self, inner: i32) -> Result<LiteralInteger, ParserError> {
    let integer_literal = LiteralInteger::new(self.current_token.span.clone(), inner);
    self.bump()?;
    Ok(integer_literal)
  }

  fn parse_expession_call(&mut self) -> Result<ExpressionCall, ParserError> {
    let mut span = self.current_token.span.clone();
    self.bump()?;
    span.end = self.current_token.span.end;
    let identifier = self.parse_identifier()?;
    Ok(ExpressionCall::new(span, identifier))
  }
}

pub struct ParserError {
  pub message: String,
  pub location: Span,
}

impl ParserError {
  fn new(message: String, location: Span) -> Self {
    Self { message, location }
  }

  fn from_lexer_error(error: LexerError) -> Self {
    Self { message: error.message, location: error.location }
  }

  fn expect_identifier(location: Span) -> Self {
    Self { message: format!("Expecting an identifier"), location }
  }

  fn expect_type_annotation(location: Span) -> Self {
    Self { message: format!("Expecting type annotation"), location }
  }

  fn unexpected_expression_token(tkn: Token) -> Self {
    Self { message: format!("Unexpected expression kind {:#?}", tkn.kind), location: tkn.span }
  }

  fn expect_either(fst: &str, scd: &str, location: Span) -> Self {
    Self { message: format!("Expecting `{}` or `{}`", fst, scd), location }
  }
}
