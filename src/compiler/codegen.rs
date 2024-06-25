use std::collections::HashMap;

use crate::ast::{Expression, FunctionDeclaration, Literal, Statment, AST};

use spider_vm::bytecode::{Bytecode, Opcode};
use spider_vm::object::Object;
use spider_vm::pool::{Pool, PoolEntry};
use spider_vm::program::{Function, Program};

pub struct CodeGenerator {
    pool: Pool,
    fns: HashMap<String, Function>,
}

impl CodeGenerator {
    pub fn make() -> Self {
        Self {
            pool: Pool::make(),
            fns: HashMap::new(),
        }
    }

    pub fn gen(&mut self, ast: AST) -> Program {
        for stmt in ast {
            self.generate_statement(stmt);
        }
        Program {
            pool: self.pool.to_owned(),
            fns: self.fns.to_owned(),
        }
    }

    fn generate_statement(&mut self, stmt: Statment) -> Bytecode {
        match stmt {
            Statment::Expression(expr) => self.generate_expression(expr),
            Statment::FunctionDeclaration(fn_decl) => self.generate_function_declaration(fn_decl),
        }
    }

    fn generate_function_declaration(&mut self, fn_decl: FunctionDeclaration) -> Bytecode {
        let mut code = Bytecode::make(vec![]);
        for stmt in fn_decl.body {
            code.instrs.extend(self.generate_statement(stmt).instrs)
        }
        code.instrs.push(Opcode::Return);
        self.fns.insert(fn_decl.name, Function { arity: 0, code });
        Bytecode::make(vec![])
    }

    fn generate_expression(&mut self, expression: Expression) -> Bytecode {
        match expression {
            Expression::Literal(literal) => self.generate_literal(literal),
            Expression::FunctionCall(fn_name) => self.generate_function_call(fn_name),
            Expression::BinaryOp(_) => self.geneate_binop(),
            _ => todo!(),
        }
    }

    fn geneate_binop(&mut self) -> Bytecode {
        let mut bytecode = Bytecode::make(vec![]);
        bytecode.instrs.push(Opcode::IAdd);
        bytecode
    }

    fn generate_function_call(&mut self, fn_name: String) -> Bytecode {
        let mut bytecode = Bytecode::make(vec![]);
        bytecode.instrs.push(Opcode::Invoke(fn_name));
        bytecode
    }

    fn generate_literal(&mut self, literal: Literal) -> Bytecode {
        let mut bytecode = Bytecode::make(vec![]);
        match literal {
            Literal::Int(x) => bytecode.instrs.push(Opcode::Bipush(x)),
            Literal::String(x) => bytecode.instrs.push(Opcode::Ldc(
                self.pool.append(PoolEntry::Object(Object::String(x))),
            )),
        };
        bytecode
    }
}
