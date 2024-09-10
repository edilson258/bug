use super::lexer::{Lexer, LexerError};
use super::token::{Token, TokenKind};
use crate::ast::*;
use crate::span::Span;

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
}

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

  fn bump_expect(&mut self, expected: TokenKind) -> Result<(), ParserError> {
    if expected != self.current_token.kind {
      return Err(ParserError::new(
        format!("Expected {:#?} but got {:#?}", expected, self.current_token.kind),
        self.current_token.span.clone(),
      ));
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
    self.bump()?; // eat 'f'

    match self.current_token.kind {
      TokenKind::Identifier(_) => {}
      _ => {
        return Err(ParserError::new(
          format!("Expected identifier after `f` but got {:#?}", self.current_token.kind),
          self.current_token.span.clone(),
        ));
      }
    };

    let name_token = self.current_token.clone();
    self.bump()?;
    self.bump_expect(TokenKind::Arrow)?;

    let mut body: Vec<Statement> = vec![];

    while TokenKind::Dot != self.current_token.kind {
      body.push(self.parse_statement()?);
    }

    self.bump_expect(TokenKind::Dot)?;

    Ok(StatementFunction::new(name_token, body))
  }

  fn parse_statement_expression(&mut self) -> Result<StatementExpression, ParserError> {
    match self.current_token.kind {
      TokenKind::At => Ok(StatementExpression::Call(self.parse_expession_call()?)),
      TokenKind::String(_) | TokenKind::Number(_) => Ok(StatementExpression::Literal(self.parse_expession_literal()?)),
      TokenKind::Plus => Ok(StatementExpression::Binary(self.parse_expression_binary()?)),
      _ => Err(ParserError::new(
        format!("Unexpected expression `{:#?}`", self.current_token.kind),
        self.current_token.span.clone(),
      )),
    }
  }

  fn parse_expression_binary(&mut self) -> Result<ExpressionBinary, ParserError> {
    let op = match self.current_token.kind {
      TokenKind::Plus => BinaryOperator::Add,
      _ => unreachable!(),
    };
    let binary_expression = ExpressionBinary::new(op, self.current_token.span.clone());
    self.bump()?;
    Ok(binary_expression)
  }

  fn parse_expession_literal(&mut self) -> Result<ExpressionLiteral, ParserError> {
    let literal_expression = match self.current_token.kind {
      TokenKind::String(_) => ExpressionLiteral::String(LiteralString::new(self.current_token.clone())),
      TokenKind::Number(_) => ExpressionLiteral::Number(NumberLiteral::new(self.current_token.clone())),
      _ => unreachable!("Expected token to be a literal"),
    };
    self.bump()?;
    Ok(literal_expression)
  }

  fn parse_expession_call(&mut self) -> Result<ExpressionCall, ParserError> {
    let mut at_span = self.current_token.span.clone();
    self.bump()?; // eat '@'

    let name_token = match self.current_token.kind {
      TokenKind::Identifier(_) => self.current_token.clone(),
      _ => {
        return Err(ParserError::new(
          format!("Expected identifier after `@` but got {:#?}", self.current_token.kind),
          self.current_token.span.clone(),
        ));
      }
    };
    self.bump()?;
    at_span.end = name_token.span.end;
    Ok(ExpressionCall::new(Token { kind: name_token.kind, span: at_span }))
  }
}
