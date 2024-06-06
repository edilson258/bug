use super::ast::{Expression, Infix, Literal, Precedence, Statment, AST};
use super::lexer::Lexer;
use super::token::Token;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    curr_token: Token,
    next_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        Self {
            lexer,
            curr_token: Token::Eof,
            next_token: Token::Eof,
        }
    }

    fn bump(&mut self) -> Result<(), String> {
        self.curr_token = self.next_token.clone();
        match self.lexer.next_token() {
            Ok(next_token) => {
                self.next_token = next_token;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Will build the AST representing the user program
    ///
    /// # Returns
    ///
    /// - `On Success`: AST
    /// - `On Error`: A string explaining the error reason
    ///
    pub fn parse(&mut self) -> Result<AST, String> {
        // Populate self.curr_token & self.next_token
        // which have Token::EOF value.
        self.bump()?;
        self.bump()?;
        let mut ast: AST = vec![];
        while self.curr_token != Token::Eof {
            let stmt = self.parse_statment()?;
            ast.push(stmt);
            self.bump()?;
        }
        Ok(ast)
    }

    fn parse_statment(&mut self) -> Result<Statment, String> {
        match self.curr_token {
            _ => match self.parse_expression(Precedence::Lowest) {
                Ok(expression) => Ok(Statment::Expression(expression)),
                Err(err) => Err(err),
            },
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let mut lhs = match self.curr_token {
            Token::Int(x) => Expression::Literal(Literal::Int(x)),
            Token::String(ref x) => Expression::Literal(Literal::String(x.clone())),
            _ => return Err(format!("Unexpected expression: {}", self.curr_token)),
        };

        while precedence < self.next_token.precedence() {
            match self.next_token {
                Token::Plus => {
                    self.bump()?;
                    lhs = self.parse_infix_expression(lhs)?;
                }
                _ => return Ok(lhs),
            }
        }

        Ok(lhs)
    }

    fn parse_infix_expression(&mut self, lhs: Expression) -> Result<Expression, String> {
        let infix = match self.curr_token {
            Token::Plus => Infix::Plus,
            _ => return Err(format!("Unexpected infix operator: {}", self.curr_token)),
        };
        let precedence = self.curr_token.precedence();
        self.bump()?;
        let rhs = self.parse_expression(precedence)?;
        Ok(Expression::Infix(Box::new(lhs), infix, Box::new(rhs)))
    }
}
