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
            Token::If => self.parse_if_statement(),
            Token::Equal => Ok(Statement::Assignment(None)),
            Token::FunctionDeclarator => self.parse_function_declaration(),
            Token::TypeString | Token::TypeInteger | Token::TypeBoolean => {
                self.parse_var_declaration()
            }
            _ => match self.parse_expression() {
                Ok(expression) => Ok(Statement::Expression(expression)),
                Err(err) => Err(err),
            },
        }
    }

    fn parse_var_declaration(&mut self) -> Result<Statement, ParserError> {
        let var_type = match &self.curr_token {
            Token::TypeString => Type::String,
            Token::TypeInteger => Type::Integer,
            Token::TypeBoolean => Type::Boolean,
            x => return Err(format!("Cannot declare variable with prefix '{}'", x)),
        };
        self.bump()?;
        let var_name = match &self.curr_token {
            Token::Identifier(ref name) => name.clone(),
            x => return Err(format!("Cannot declare variable with name '{}'", x)),
        };
        Ok(Statement::VariableDeclaration(VariableDeclaration {
            type_: var_type,
            name: var_name,
        }))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.bump_expected(Token::If)?;
        self.bump_expected(Token::Arrow)?;
        let if_block = self.parse_block_statement()?;
        if let Token::Else = self.next_token {
            self.bump_expected(Token::Semicolon)?;
            self.bump_expected(Token::Else)?;
            self.bump_expected(Token::Arrow)?;
            let else_block = self.parse_block_statement()?;
            Ok(Statement::If(if_block, Some(else_block)))
        } else {
            Ok(Statement::If(if_block, None))
        }
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
            Token::True => Ok(Expression::Literal(Literal::Boolean(true))),
            Token::False => Ok(Expression::Literal(Literal::Boolean(false))),
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

#[cfg(test)]
mod tests {
    use super::{Parser, Statement};
    use crate::frontend::{
        lexer::Lexer,
        parser::{BinaryOp, Expression, FnParam, Literal},
    };
    use bug::Type;

    #[test]
    fn missing_function_name_on_call() {
        let input = ".";
        let input = input.chars().collect::<Vec<char>>();
        let mut l = Lexer::new(&input);
        let mut p = Parser::new(&mut l);
        match p.parse() {
            Err(_) => {}
            Ok(_) => panic!("Expected error: Missing function's name of call expression"),
        };
    }

    #[test]
    fn ensure_function_is_well_declared() {
        let input = "f double(int x) int -> x x +;";

        let input = input.chars().collect::<Vec<char>>();
        let mut l = Lexer::new(&input);
        let mut p = Parser::new(&mut l);

        let fn_decl = match p.parse() {
            Ok(x) => x[0].clone(),
            Err(e) => panic!("{}", e),
        };
        let fn_decl = match fn_decl {
            Statement::FunctionDeclaration(fn_decl) => fn_decl,
            _ => panic!("Expected a function declaration node"),
        };

        assert_eq!(
            "double".to_string(),
            fn_decl.name,
            "Function name must be 'double'"
        );
        assert_eq!(
            Type::Integer,
            fn_decl.return_type,
            "Return type must be Integer"
        );
        assert_eq!(3, fn_decl.body.len(), "Function body must has 3 statements");

        let expected_param_list = vec![FnParam {
            name: "x".to_string(),
            type_: Type::Integer,
        }];

        assert_eq!(expected_param_list.len(), fn_decl.params.len());

        expected_param_list.iter().zip(fn_decl.params).for_each(
            |(expected_param, provided_param)| assert_eq!(*expected_param, provided_param),
        );

        match fn_decl.body[0].clone() {
            Statement::Expression(Expression::Identifier(x)) => {
                assert_eq!("x".to_string(), x)
            }
            _ => panic!("First statement must be Identifier 'x'"),
        }

        match fn_decl.body[1].clone() {
            Statement::Expression(Expression::Identifier(x)) => {
                assert_eq!("x".to_string(), x)
            }
            _ => panic!("Second statement must be Identifier 'x'"),
        }

        match fn_decl.body[2].clone() {
            Statement::Expression(Expression::BinaryOp(BinaryOp::Plus(_))) => {}
            _ => panic!("Last statement must be BinaryOp 'plus'"),
        }
    }

    #[test]
    fn ensure_call_is_well_formed() {
        let input = ".sum";

        let input = input.chars().collect::<Vec<char>>();
        let mut l = Lexer::new(&input);
        let mut p = Parser::new(&mut l);

        let ast = match p.parse() {
            Ok(ast) => ast,
            Err(e) => panic!("{}", e),
        };

        assert_eq!(1, ast.len());

        match ast[0].clone() {
            Statement::Expression(Expression::FunctionCall(callee)) => {
                assert_eq!("sum", &callee)
            }
            x => panic!("Expected function call expression, but got {:#?}", x),
        }
    }

    #[test]
    fn ensure_int_literal_expression_is_well_formed() {
        let input = "69";

        let input = input.chars().collect::<Vec<char>>();
        let mut l = Lexer::new(&input);
        let mut p = Parser::new(&mut l);

        let ast = match p.parse() {
            Ok(ast) => ast,
            Err(e) => panic!("{}", e),
        };

        assert_eq!(1, ast.len());

        match ast[0].clone() {
            Statement::Expression(Expression::Literal(Literal::Int(69))) => {}
            _ => panic!("Expected int literal expression '69'"),
        }
    }

    #[test]
    fn ensure_str_literal_expression_is_well_formed() {
        let input = "\"Hi\"";

        let input = input.chars().collect::<Vec<char>>();
        let mut l = Lexer::new(&input);
        let mut p = Parser::new(&mut l);

        let ast = match p.parse() {
            Ok(ast) => ast,
            Err(e) => panic!("{}", e),
        };

        assert_eq!(1, ast.len());

        match ast[0].clone() {
            Statement::Expression(Expression::Literal(Literal::String(x))) => assert_eq!("Hi", &x),
            _ => panic!("Expected int literal expression '69'"),
        }
    }
}
