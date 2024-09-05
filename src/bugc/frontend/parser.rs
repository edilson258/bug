use crate::ast::*;

use super::{
  lexer::Lexer,
  token::{Token, TokenKind},
};

pub struct Parser<'a> {
  current_token: Token,
  next_token: Token,
  lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
    Self { lexer, current_token: Token::default(), next_token: Token::default() }
  }

  pub fn parse(&mut self) -> Ast {
    self.bump();
    self.bump();

    let mut ast: Ast = vec![];

    while self.current_token.kind != TokenKind::Eof {
      ast.push(self.parse_statement());
    }

    return ast;
  }

  fn bump(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.next_token();
  }

  fn bump_expect(&mut self, expected: TokenKind) {
    if expected != self.current_token.kind {
      panic!("Expected {:#?} got {:#?}", expected, self.current_token);
    }
    self.bump();
  }

  fn parse_statement(&mut self) -> Statement {
    match self.current_token.kind {
      TokenKind::Function => Statement::Function(self.parse_statement_function()),
      _ => Statement::Expression(self.parse_statement_expression()),
    }
  }

  fn parse_statement_function(&mut self) -> StatementFunction {
    self.bump(); // eat 'f'

    match self.current_token.kind {
      TokenKind::Identifier(_) => {}
      _ => {
        panic!("[ERROR]: Expected identifier after `f`");
      }
    };

    let name_token = self.current_token.clone();
    self.bump();
    self.bump_expect(TokenKind::Arrow);

    let mut body: Vec<Statement> = vec![];

    while TokenKind::Dot != self.current_token.kind {
      body.push(self.parse_statement());
    }

    self.bump_expect(TokenKind::Dot);

    StatementFunction::new(name_token, body)
  }

  // f main -> "Hello, world!" @write.

  fn parse_statement_expression(&mut self) -> StatementExpression {
    match self.current_token.kind {
      TokenKind::At => StatementExpression::Call(self.parse_expession_call()),
      TokenKind::String(_) => StatementExpression::Literal(self.parse_expession_literal()),
      _ => panic!("Unexpexted expression"),
    }
  }

  fn parse_expession_literal(&mut self) -> ExpressionLiteral {
    let literal = match self.current_token.kind {
      TokenKind::String(_) => ExpressionLiteral::String(LiteralString::new(self.current_token.clone())),
      _ => unreachable!("Expected token to be a literal"),
    };
    self.bump();

    literal
  }

  fn parse_expession_call(&mut self) -> ExpressionCall {
    let at_location = self.current_token.location.clone();
    self.bump(); // eat '@'

    let name_token = match self.current_token.kind {
      TokenKind::Identifier(_) => self.current_token.clone(),
      _ => panic!("Expected identifier after `f`"),
    };
    self.bump();

    ExpressionCall::new(Token { kind: name_token.kind, location: at_location + name_token.location })
  }
}
