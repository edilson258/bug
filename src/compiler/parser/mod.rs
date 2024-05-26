use core::fmt;

use super::{
    ast::{Expression, Infix, Literal, Precedence, Statment, AST},
    lexer::Lexer,
    token::Token,
};

#[derive(Clone)]
enum PEKind {
    Syntax,
}

impl fmt::Display for PEKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax => write!(f, "[Syntax Error]"),
        }
    }
}

#[derive(Clone)]
pub struct PError {
    kind: PEKind,
    msg: String,
}

impl fmt::Display for PError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

type Perrors = Vec<PError>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    curr_token: Token,
    next_token: Token,
    errors: Vec<PError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let mut p = Parser {
            lexer,
            curr_token: Token::Eof,
            next_token: Token::Eof,
            errors: vec![],
        };

        p.bump();
        p.bump();

        p
    }

    fn bump(&mut self) {
        self.curr_token = self.next_token.clone();
        let next_token = self.lexer.next_token();

        if next_token.is_err() {
            eprintln!("{:?}", next_token.err());
            return;
        }

        self.next_token = next_token.unwrap();
    }

    pub fn parse(&mut self) -> Result<AST, Perrors> {
        let mut ast: AST = vec![];
        while self.curr_token != Token::Eof {
            match self.parse_stmt() {
                Ok(stmt) => ast.push(stmt),
                Err(err) => self.errors.push(err),
            }
            self.bump();
        }
        if self.errors.is_empty() {
            return Ok(ast);
        }
        Err(self.errors.clone())
    }

    fn parse_stmt(&mut self) -> Result<Statment, PError> {
        match self.curr_token {
            _ => match self.parse_expr(Precedence::Lowest) {
                Ok(expression) => Ok(Statment::Expression(expression)),
                Err(err) => Err(err),
            },
        }
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Result<Expression, PError> {
        let mut lhs = match self.curr_token {
            Token::Int(x) => Expression::Literal(Literal::Int(x)),
            _ => {
                return Err(PError {
                    kind: PEKind::Syntax,
                    msg: format!("Unexpected: {}", self.curr_token),
                })
            }
        };

        while precedence < self.next_token.precedence() {
            match self.next_token {
                Token::Plus => {
                    self.bump();
                    lhs = self.parse_infix(lhs)?;
                }
                _ => return Ok(lhs),
            }
        }

        Ok(lhs)
    }

    fn parse_infix(&mut self, lhs: Expression) -> Result<Expression, PError> {
        let infix = match self.curr_token {
            Token::Plus => Infix::Plus,
            _ => {
                return Err(PError {
                    kind: PEKind::Syntax,
                    msg: format!("expected an infix operator"),
                })
            }
        };
        let precedence = self.curr_token.precedence();
        self.bump();
        let rhs = self.parse_expr(precedence)?;
        Ok(Expression::Infix(Box::new(lhs), infix, Box::new(rhs)))
    }
}
