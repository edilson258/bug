use crate::analysis::Type;
use crate::ast::{BlockStatment, FunctionCall, FunctionDeclaration};

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

    fn bump_expected(&mut self, token: Token) -> Result<(), String> {
        if self.curr_token == token {
            self.bump()?;
            Ok(())
        } else {
            Err(format!("Expected {} but got {}", token, self.curr_token))
        }
    }

    fn is_curr_token(&self, token: Token) -> bool {
        self.curr_token == token
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
        }
        Ok(ast)
    }

    fn parse_statment(&mut self) -> Result<Statment, String> {
        match self.curr_token {
            Token::Dot => self.parse_function_definition(),
            _ => match self.parse_expression(Precedence::Lowest) {
                Ok(expression) => Ok(Statment::Expression(expression)),
                Err(err) => Err(err),
            },
        }
    }

    fn parse_function_definition(&mut self) -> Result<Statment, String> {
        self.bump_expected(Token::Dot)?;
        let name = match self.curr_token {
            Token::Identifier(ref name) => name.clone(),
            _ => return Err(format!("'.' must follow an identifier")),
        };
        self.bump()?;

        // @TODO: handle function's params.
        self.bump_expected(Token::Lparen)?;
        self.bump_expected(Token::Rparen)?;

        let return_type = match self.curr_token {
            Token::Arrow => Type::Void,
            _ => todo!(),
        };
        self.bump()?;

        let body = self.parse_block_statment()?;
        Ok(Statment::FunctionDeclaration(FunctionDeclaration::make(
            name,
            return_type,
            body,
        )))
    }

    fn parse_block_statment(&mut self) -> Result<BlockStatment, String> {
        let mut block: BlockStatment = vec![];
        // every block must end  with Semicolon.
        while !self.is_curr_token(Token::Semicolon) {
            block.push(self.parse_statment()?);
        }
        self.bump_expected(Token::Semicolon)?;
        Ok(block)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let mut lhs = match self.curr_token {
            Token::Int(x) => Expression::Literal(Literal::Int(x)),
            Token::String(ref x) => Expression::Literal(Literal::String(x.clone())),
            Token::Identifier(ref identifier) => Expression::Identifier(identifier.clone()),
            _ => return Err(format!("Unexpected expression: {}", self.curr_token)),
        };

        // write("Hello");
        while precedence < self.next_token.precedence() {
            match self.next_token {
                Token::Plus => {
                    self.bump()?;
                    lhs = self.parse_infix_expression(lhs)?;
                }
                Token::Lparen => {
                    self.bump()?;
                    match lhs {
                        Expression::Identifier(name) => {
                            lhs = self.parse_function_call(name)?;
                        }
                        _ => return Err(format!("'(' must follow an identifier")),
                    }
                }
                _ => return Ok(lhs),
            }
        }

        Ok(lhs)
    }

    fn parse_function_call(&mut self, fn_name: String) -> Result<Expression, String> {
        self.bump_expected(Token::Lparen)?;
        let arg = self.parse_expression(Precedence::Call)?;
        self.bump()?;
        self.bump_expected(Token::Rparen)?;
        Ok(Expression::FunctionCall(FunctionCall::make(
            fn_name,
            vec![arg],
        )))
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
