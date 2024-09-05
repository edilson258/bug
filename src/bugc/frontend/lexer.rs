use super::token::{Location, Token, TokenKind};

pub struct Lexer<'a> {
  input: &'a str,
  _file_name: &'a str,
  cursor: usize,
  line: usize,
  colm: usize,
  location: Location,
}

impl<'a> Lexer<'a> {
  pub fn new(file_name: &'a str, input: &'a str) -> Self {
    Lexer { _file_name: &file_name, input, cursor: 0, line: 1, colm: 1, location: Location::default() }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    self.update_location();

    if self.is_eof() {
      self.advance_one();
      return Token { kind: TokenKind::Eof, location: self.get_location() };
    }

    match self.peek_one() {
      '@' => self.read_simple_token(TokenKind::At),
      '.' => self.read_simple_token(TokenKind::Dot),
      ';' => self.read_simple_token(TokenKind::Semicolon),
      '-' => self.read_check_ahead("->", TokenKind::Minus, TokenKind::Arrow),
      '"' => self.read_string(),
      'a'..='z' | 'A'..='Z' | '_' => self.read_keyword_or_identifier(),
      '0'..='9' => self.read_number(),
      _ => {
        panic!("[ERROR]: Unexpected token {}", self.peek_one());
      }
    }
  }

  fn read_number(&mut self) -> Token {
    let num = self.chop_while(|x| x.is_numeric() || x == '.');
    let num = num.parse::<f32>().unwrap_or_else(|err| panic!("[ERROR]: Couldn't parse string to number {err}"));
    Token::new(TokenKind::Number(num), self.get_location())
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

    Token::new(TokenKind::String(text), self.get_location())
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
    Token::keyword_or_identifier(label, self.get_location())
  }

  fn read_check_ahead(&mut self, expected: &str, simple: TokenKind, complex: TokenKind) -> Token {
    if self.starts_with(expected) {
      self.advance_may(expected.len());
      return Token::new(complex, self.get_location());
    }
    Token::new(simple, self.get_location())
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
    let token = Token::new(kind, self.get_location());
    return token;
  }

  fn update_location(&mut self) {
    self.location.line = self.line;
    self.location.column = self.colm;
    self.location.start = self.cursor;
  }

  fn get_location(&mut self) -> Location {
    self.location.end = self.cursor - 1;
    return self.location.clone();
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

#[cfg(test)]
mod tests {}
