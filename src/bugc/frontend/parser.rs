use bug::Type;

use super::lexer::Lexer;
use super::Token;
use crate::ast::*;

type ParserError = String;

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

    fn bump(&mut self) -> Result<(), ParserError> {
        self.curr_token = self.next_token.clone();
        match self.lexer.next_token() {
            Ok(next_token) => {
                self.next_token = next_token;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn bump_expected(&mut self, token: Token) -> Result<(), ParserError> {
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

    pub fn parse(&mut self) -> Result<AST, ParserError> {
        self.bump()?;
        self.bump()?;

        let mut ast: AST = vec![];

        while self.curr_token != Token::Eof {
            let stmt = self.parse_statement()?;
            ast.push(stmt);
            self.bump()?;
            if self.curr_token == Token::Semicolon {
                self.bump()?;
            }
        }

        Ok(ast)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.curr_token {
            Token::FunctionDeclarator => self.parse_function_declaration(),
            Token::If => self.parse_if_statement(),
            _ => match self.parse_expression() {
                Ok(expression) => Ok(Statement::Expression(expression)),
                Err(err) => Err(err),
            },
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.bump_expected(Token::If)?;
        self.bump_expected(Token::Arrow)?;
        let block = self.parse_block_statement()?;
        Ok(Statement::If(block))
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, ParserError> {
        self.bump_expected(Token::FunctionDeclarator)?;
        let name = match self.curr_token {
            Token::Identifier(ref name) => name.clone(),
            _ => return Err(format!("'f' must follow an identifier")),
        };
        self.bump()?;

        let params: FnParams = self.parse_function_params()?;
        let return_type = self.parse_function_return_type()?;

        self.bump_expected(Token::Arrow)?;

        let body = self.parse_block_statement()?;

        Ok(Statement::FunctionDeclaration(FunctionDeclaration {
            name,
            params,
            return_type,
            body,
        }))
    }

    fn parse_function_return_type(&mut self) -> Result<Type, ParserError> {
        let type_ = match self.curr_token {
            Token::Arrow => Type::Void,
            Token::TypeInteger => Type::Integer,
            Token::TypeString => Type::String,
            _ => {
                return Err(format!(
                    "Expected return type annotation, but provided '{}'",
                    self.curr_token
                ))
            }
        };

        if self.curr_token != Token::Arrow {
            self.bump()?;
        }

        Ok(type_)
    }

    fn parse_function_params(&mut self) -> Result<FnParams, ParserError> {
        let mut params: FnParams = vec![];
        if self.curr_token == Token::Arrow {
            return Ok(params);
        }
        self.bump_expected(Token::Lparen)?;
        while self.curr_token != Token::Rparen {
            let param_type = match self.curr_token {
                Token::TypeString => Type::String,
                Token::TypeInteger => Type::Integer,
                _ => {
                    return Err(format!(
                        "Expected param type, but provided '{}'",
                        self.curr_token
                    ))
                }
            };
            self.bump()?;
            let param_name = match self.curr_token {
                Token::Identifier(ref name) => name.clone(),
                _ => {
                    return Err(format!(
                        "Expected param name, but provided '{}'",
                        self.curr_token
                    ))
                }
            };
            self.bump()?;
            params.push(FnParam {
                name: param_name,
                type_: param_type,
            });
            match self.curr_token {
                Token::Rparen => break,
                Token::Comma => self.bump()?,
                _ => return Err(format!("Function params must be separated by ','")),
            };
        }
        self.bump_expected(Token::Rparen)?;
        Ok(params)
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        let mut block: BlockStatement = vec![];
        while !self.is_curr_token(Token::Semicolon) {
            block.push(self.parse_statement()?);
            self.bump()?;
        }
        Ok(block)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.curr_token {
            Token::Int(x) => Ok(Expression::Literal(Literal::Int(x))),
            Token::String(ref x) => Ok(Expression::Literal(Literal::String(x.clone()))),
            Token::Identifier(ref identifier) => Ok(Expression::Identifier(identifier.clone())),
            Token::Dot => self.parse_function_call(),
            Token::Plus => Ok(Expression::BinaryOp(BinaryOp::Plus(None))),
            Token::GratherThan => Ok(Expression::BinaryOp(BinaryOp::GratherThan(None))),
            Token::Return => Ok(Expression::Return(None)),
            _ => return Err(format!("Unexpected expression: {}", self.curr_token)),
        }
    }

    fn parse_function_call(&mut self) -> Result<Expression, ParserError> {
        self.bump_expected(Token::Dot)?;
        match &self.curr_token {
            Token::Identifier(ref fn_name) => Ok(Expression::FunctionCall(fn_name.clone())),
            _ => Err(format!("Missing function's name")),
        }
    }
}
