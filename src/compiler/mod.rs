mod analysis;
mod ast;
mod lexer;
mod parser;
mod token;

use std::process::exit;

use crate::analysis::Analiser;
use crate::lexer::Lexer;
use crate::parser::Parser;
use ast::{Expression, Infix, Literal, Statment, AST};

use spider_vm::bytecode::{Bytecode, Opcode};
use spider_vm::object::Object;
use spider_vm::pool::{FunctionRef, Pool, PoolEntry};
use spider_vm::program::{Function, Program};

struct CodeGenerator {
    pool: Pool,
    fns: Vec<Function>,
    bytecode: Vec<Opcode>,
}

impl CodeGenerator {
    pub fn make() -> Self {
        Self {
            pool: Pool::make(),
            fns: vec![],
            bytecode: vec![],
        }
    }

    pub fn gen(&mut self, ast: AST) -> Program {
        for stmt in ast {
            self.generate_stmt(stmt);
        }

        let main_fn = Function {
            fn_pool_ref: self
                .pool
                .append(PoolEntry::FunctionRef(FunctionRef::make(0, 0, 0))),
            arity: 0,
            code: Bytecode::make(self.bytecode.clone()),
            max_stack: 2,
            max_locals: 0,
        };

        Program::make(self.pool.clone(), vec![main_fn])
    }

    fn generate_stmt(&mut self, stmt: Statment) {
        match stmt {
            Statment::Expression(expr) => self.generate_expression(expr),
        }
    }

    fn generate_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Literal(literal) => self.generate_literal(literal),
            Expression::Infix(lhs, infix, rhs) => self.generate_infix(*lhs, infix, *rhs),
        }
    }

    fn generate_infix(&mut self, lhs: Expression, infix: Infix, rhs: Expression) {
        let operands_type = lhs.ask_type();
        self.generate_expression(lhs);
        self.generate_expression(rhs);

        match infix {
            Infix::Plus => match operands_type {
                analysis::Type::Integer => self.append_instruction(Opcode::IAdd),
                _ => todo!(),
            },
        };
    }

    fn generate_literal(&mut self, literal: Literal) {
        match literal {
            Literal::Int(x) => self.append_instruction(Opcode::Bipush(x)),
            Literal::String(x) => {
                let index = self.pool.append(PoolEntry::Object(Object::String(x)));
                self.append_instruction(Opcode::Ldc(index));
            }
        }
    }

    fn append_instruction(&mut self, opcode: Opcode) {
        self.bytecode.push(opcode);
    }
}

fn main() {
    let input = "1 + 2".to_string().chars().collect::<Vec<char>>();
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);

    let mut ast = match p.parse() {
        Ok(ast) => ast,
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            exit(1)
        }
    };

    let mut analiser = Analiser::make();
    match analiser.analise(&mut ast) {
        Ok(()) => {}
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
            exit(1)
        }
    }

    let mut generator = CodeGenerator::make();
    let program = generator.gen(ast);

    println!("{:#?}", program);
}
