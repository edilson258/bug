use super::token::{Token, TokenKind};
use crate::span::Span;

pub struct Lexer<'a> {
  input: &'a str,
  file_name: &'a str,
  cursor: usize,
  line: usize,
  colm: usize,
  span: Span,
}

impl<'a> Lexer<'a> {
  pub fn new(file_name: &'a str, input: &'a str) -> Self {
    Lexer { file_name, input, cursor: 0, line: 1, colm: 1, span: Span::default() }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    self.update_span();

    if self.is_eof() {
      self.advance_one();
      return Token { kind: TokenKind::Eof, span: self.get_span() };
    }

    match self.peek_one() {
      '"' => self.read_string(),
      '0'..='9' => self.read_number(),
      '@' => self.read_simple_token(TokenKind::At),
      '.' => self.read_simple_token(TokenKind::Dot),
      '+' => self.read_simple_token(TokenKind::Plus),
      ';' => self.read_simple_token(TokenKind::Semicolon),
      '-' => self.read_check_ahead("->", TokenKind::Minus, TokenKind::Arrow),
      'a'..='z' | 'A'..='Z' | '_' => self.read_keyword_or_identifier(),
      _ => {
        panic!("[ERROR]: Unexpected token {}", self.peek_one());
      }
    }
  }

  fn read_number(&mut self) -> Token {
    let num = self.chop_while(|x| x.is_numeric() || x == '.');
    let num = num.parse::<f32>().unwrap_or_else(|err| panic!("[ERROR]: Couldn't parse string to number {err}"));
    Token::new(TokenKind::Number(num), self.get_span())
  }

  fn read_string(&mut self) -> Token {
    self.advance_one();
    let text_start = self.cursor;

    loop {
      if self.peek_one() == '"' {
        break;
      }

      if self.is_eof() || self.peek_one() == '\n' {
        panic!("[ERROR]: Unterminated string literal");
      }

      self.advance_one();
    }

    let text = self.input[text_start..self.cursor].to_string();
    self.advance_one(); // eat right '"'

    Token::new(TokenKind::String(text), self.get_span())
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
