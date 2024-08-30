#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Eof,

  String(String),
  Identifier(String),

  Function,

  At,
  Dot,
  Arrow,
  Semicolon,

  Minus,
}

#[derive(Debug, Clone)]
pub struct Location {
  pub line: usize,
  pub column: usize,
  pub start: usize,
  pub end: usize,
}

impl Default for Location {
  fn default() -> Self {
    Self { line: 0, column: 0, start: 0, end: 0 }
  }
}

#[derive(Debug, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub location: Location,
}

impl Token {
  pub fn new(kind: TokenKind, location: Location) -> Self {
    Self { kind, location }
  }

  pub fn keyword_or_identifier(label: String, location: Location) -> Self {
    match label.as_str() {
      "f" => Token::new(TokenKind::Function, location),
      _ => Token::new(TokenKind::Identifier(label), location),
    }
  }
}

impl Default for Token {
  fn default() -> Self {
    Token { kind: TokenKind::Eof, location: Location::default() }
  }
}
