use super::token::{Token, TokenKind};
use crate::span::Span;

pub struct LexerError {
  pub message: String,
  pub location: Span,
}

impl LexerError {
  fn new(message: String, location: Span) -> Self {
    Self { message, location }
  }
}

pub struct Lexer<'a> {
  line: usize,
  colm: usize,
  input: &'a str,
  cursor: usize,
  span: Span,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Lexer { input, cursor: 0, line: 1, colm: 1, span: Span::default() }
  }

  pub fn next_token(&mut self) -> Result<Token, LexerError> {
    self.skip_whitespace();
    self.update_span();
    if self.is_eof() {
      return Ok(Token::new(TokenKind::Eof, self.get_span()));
    }
    match self.peek_one() {
      '@' => Ok(self.read_simple_token(TokenKind::At)),
      '.' => Ok(self.read_simple_token(TokenKind::Dot)),
      '+' => Ok(self.read_simple_token(TokenKind::Plus)),
      ';' => Ok(self.read_simple_token(TokenKind::Semicolon)),
      '-' => Ok(self.read_check_ahead("->", TokenKind::Minus, TokenKind::Arrow)),
      '"' => self.read_string(),
      '0'..='9' => self.read_number(),
      'a'..='z' | 'A'..='Z' | '_' => Ok(self.read_keyword_or_identifier()),
      _ => Err(LexerError::new(format!("Unexpected token `{}`", self.peek_one()), self.get_span())),
    }
  }

  fn read_number(&mut self) -> Result<Token, LexerError> {
    let raw_number = self.chop_while(|x| x.is_numeric() || x == '.');
    let number = match raw_number.parse::<f32>() {
      Ok(num) => num,
      Err(err) => return Err(LexerError::new(format!("Couldn't parse number literal: {err}"), self.get_span())),
    };
    Ok(Token::new(TokenKind::Number(number), self.get_span()))
  }

  fn read_string(&mut self) -> Result<Token, LexerError> {
    self.advance_one();
    let text_start = self.cursor;
    loop {
      if self.peek_one() == '"' {
        break;
      }
      if self.is_eof() || self.peek_one() == '\n' {
        return Err(LexerError::new(format!("Unterminated string literal"), self.get_span()));
      }
      self.advance_one();
    }
    let text = self.input[text_start..self.cursor].to_string();
    self.advance_one(); // eat right '"'
    Ok(Token::new(TokenKind::String(text), self.get_span()))
  }

  fn advance_one(&mut self) {
    if let Some(c) = self.input[self.cursor..].chars().next() {
      if c == '\n' {
        self.line += 1;
        self.colm = 1;
      } else {
        self.colm += 1;
      }
      self.cursor += 1;
    }
  }

  fn read_keyword_or_identifier(&mut self) -> Token {
    let label = self.chop_while(|x| x.is_alphanumeric() || x == '_');
    Token::keyword_or_identifier(label, self.get_span())
  }

  fn read_check_ahead(&mut self, expected: &str, simple: TokenKind, complex: TokenKind) -> Token {
    if self.starts_with(expected) {
      self.advance_may(expected.len());
      return Token::new(complex, self.get_span());
    }
    Token::new(simple, self.get_span())
  }

  fn advance_may(&mut self, count: usize) {
    for _ in 0..count {
      self.advance_one();
    }
  }

  fn starts_with(&self, s: &str) -> bool {
    self.input[self.cursor..].starts_with(s)
  }

  fn read_simple_token(&mut self, kind: TokenKind) -> Token {
    self.advance_one();
    let token = Token::new(kind, self.get_span());
    return token;
  }

  fn update_span(&mut self) {
    self.span.line = self.line;
    self.span.column = self.colm;
    self.span.start = self.cursor;
  }

  fn get_span(&mut self) -> Span {
    self.span.end = self.cursor - 1;
    return self.span.clone();
  }

  fn skip_whitespace(&mut self) {
    self.chop_while(|x| x.is_whitespace());
  }

  fn is_eof(&self) -> bool {
    self.cursor >= self.input.len()
  }

  fn peek_one(&self) -> char {
    self.input[self.cursor..].chars().next().unwrap_or('\0')
  }

  fn chop_while<P>(&mut self, mut predicate: P) -> String
  where
    P: FnMut(char) -> bool,
  {
    let start = self.cursor;
    while !self.is_eof() && predicate(self.peek_one()) {
      self.advance_one();
    }
    return self.input[start..self.cursor].to_string();
  }
}
