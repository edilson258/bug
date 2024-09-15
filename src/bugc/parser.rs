use super::lexer::Lexer;
use super::token::{Token, TokenKind};
use crate::ast::*;
use crate::highlighter::highlight_error;
use crate::span::Span;
use bug::Type;

pub struct Parser<'a> {
  raw: &'a str,
  file_path: &'a str,
  current_token: Token,
  next_token: Token,
  lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(file_path: &'a str, raw: &'a str, lexer: &'a mut Lexer<'a>) -> Self {
    Self { file_path, raw, lexer, current_token: Token::default(), next_token: Token::default() }
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
      Err(error) => return Err(self.error(&error.message, &error.location)),
    };
    Ok(())
  }

  fn bump_expect(&mut self, expected: TokenKind, message: &str) -> Result<(), ParserError> {
    if expected != self.current_token.kind {
      return Err(self.error(message, &self.current_token.span));
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
        _ => return Err(self.error_expect_either(")", ",", &self.current_token.span)),
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
      TokenKind::TypeStr => Type::String,
      _ => return Err(self.error_expect_type_annotation(&self.current_token.span)),
    };
    self.bump()?;
    Ok(typ)
  }

  fn parse_identifier(&mut self) -> Result<Identifier, ParserError> {
    let identifier = match &self.current_token.kind {
      TokenKind::Identifier(label) => Identifier::new(self.current_token.span.clone(), label.clone()),
      _ => return Err(self.error_expect_identifier(&self.current_token.span)),
    };
    self.bump()?;
    Ok(identifier)
  }

  fn parse_statement_expression(&mut self) -> Result<StatementExpression, ParserError> {
    match self.current_token.kind {
      TokenKind::At => Ok(StatementExpression::Call(self.parse_expession_call()?)),
      TokenKind::String(_) | TokenKind::Integer(_) => Ok(StatementExpression::Literal(self.parse_expession_literal()?)),
      TokenKind::Plus | TokenKind::RightAngle => Ok(StatementExpression::Binary(self.parse_expression_binary()?)),
      TokenKind::Identifier(_) => Ok(StatementExpression::Identifier(self.parse_expression_identifier()?)),
      TokenKind::QuestionMark => Ok(StatementExpression::Ternary(self.parse_expression_ternary()?)),
      _ => Err(self.error_unexpected_expression(&self.current_token.span)),
    }
  }

  fn parse_expression_ternary(&mut self) -> Result<ExpressionTernary, ParserError> {
    let mut span = self.current_token.span.clone();
    self.bump()?; // eat `?`
    let consequence = self.parse_statement_expression()?;
    self.bump_expect(TokenKind::Colon, "Expecting `:` after consequence expression of the ternary operator")?;
    let alternative = self.parse_statement_expression()?;
    span.end = alternative.get_span().end;
    Ok(ExpressionTernary::new(consequence, alternative, span))
  }

  fn parse_expression_identifier(&mut self) -> Result<ExpressionIdentifier, ParserError> {
    let name = match &self.current_token.kind {
      TokenKind::Identifier(identifier) => identifier.clone(),
      _ => unreachable!("Invalid identifier expression"),
    };
    let span = self.current_token.span.clone();
    self.bump()?;
    Ok(ExpressionIdentifier::new(name, span))
  }

  fn parse_expression_binary(&mut self) -> Result<ExpressionBinary, ParserError> {
    let op = match self.current_token.kind {
      TokenKind::Plus => BinaryOperator::Plus,
      TokenKind::Minus => BinaryOperator::Minus,
      TokenKind::RightAngle => BinaryOperator::GratherThan,
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
  message: String,
}

impl core::fmt::Display for ParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl<'a> Parser<'a> {
  fn error_expect_identifier(&self, span: &Span) -> ParserError {
    self.error("Expecting an identifier", span)
  }

  fn error_unexpected_expression(&self, span: &Span) -> ParserError {
    self.error("Unexpected expression", span)
  }

  fn error_expect_type_annotation(&self, span: &Span) -> ParserError {
    self.error("Expecting type annotation", span)
  }

  fn error_expect_either(&self, fst: &str, scd: &str, span: &Span) -> ParserError {
    self.error(&format!("Expecting either `{}` or `{}`", fst, scd), span)
  }

  fn error(&self, message: &str, span: &Span) -> ParserError {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(message);
    error.push_str("\n\n");
    error.push_str(&&highlight_error(&self.raw, span.start, span.end));
    error.push('\n');
    ParserError { message: error }
  }

  fn error_header(&self, span: &Span) -> String {
    format!(
      "\x1b[38;5;4m{}\x1b[0m:\x1b[38;5;5m{}\x1b[0m:\x1b[38;5;5m{}\x1b[0m\x1b[1;31m ERROR\x1b[0m ",
      self.file_path, span.line, span.column
    )
  }
}
