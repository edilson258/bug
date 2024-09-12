use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Eof,

  Integer(i32),
  String(String),

  Identifier(String),

  Function,

  At,
  Dot,
  Arrow,
  Comma,
  Semicolon,
  LeftParent,
  RightParent,

  TypeInt,
  TypeVoid,

  Minus,
  Plus,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

impl Token {
  pub fn new(kind: TokenKind, span: Span) -> Self {
    Self { kind, span }
  }

  pub fn keyword_or_identifier(label: String, span: Span) -> Self {
    match label.as_str() {
      "fn" => Token::new(TokenKind::Function, span),
      "int" => Token::new(TokenKind::TypeInt, span),
      "void" => Token::new(TokenKind::TypeVoid, span),
      _ => Token::new(TokenKind::Identifier(label), span),
    }
  }
}

impl Default for Token {
  fn default() -> Self {
    Token { kind: TokenKind::Eof, span: Span::default() }
  }
}
