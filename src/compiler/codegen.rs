use std::collections::HashMap;

use crate::ast::{BinaryOp, Expression, FunctionDeclaration, Literal, Statment, AST};

use spider_vm::bytecode::{Bytecode, Opcode};
use spider_vm::object::Object;
use spider_vm::pool::{Pool, PoolEntry};
use spider_vm::program::{DefinedFn, Program};
use spider_vm::stdlib::Type;

struct Local {
    index: usize,
    type_: Type,
}

impl Local {
    fn make(index: usize, type_: Type) -> Self {
        Self { index, type_ }
    }
}

pub struct CodeGenerator {
    pool: Pool,
    fns: HashMap<String, DefinedFn>,
    locals: HashMap<String, Local>,
}

impl CodeGenerator {
    pub fn make() -> Self {
        Self {
            pool: Pool::make(),
            fns: HashMap::new(),
            locals: HashMap::new(),
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
        let arity = fn_decl.params.len();
        for (i, p) in fn_decl.params.into_iter().enumerate() {
            self.locals.insert(p.name, Local::make(i, p.type_));
        }
        for stmt in fn_decl.body {
            code.instrs.extend(self.generate_statement(stmt).instrs)
        }

        match fn_decl.return_type {
            Type::Integer => code.instrs.push(Opcode::IReturn),
            _ => code.instrs.push(Opcode::Return),
        }

        self.fns.insert(
            fn_decl.name,
            DefinedFn {
                arity,
                code,
                max_locals: arity,
            },
        );
        self.locals.clear();
        Bytecode::make(vec![])
    }

    fn generate_expression(&mut self, expression: Expression) -> Bytecode {
        match expression {
            Expression::Literal(literal) => self.generate_literal(literal),
            Expression::FunctionCall(fn_name) => self.generate_function_call(fn_name),
            Expression::BinaryOp(binop) => self.generate_binop(binop),
            Expression::Identifier(ident) => self.generate_identifier(ident),
        }
    }

    fn generate_identifier(&mut self, ident: String) -> Bytecode {
        let mut bytecode = Bytecode::make(vec![]);
        let local = self
            .locals
            .get(&ident)
            .expect(&format!("Expected '{}' to a local", &ident));
        match local.type_ {
            Type::Integer => bytecode.instrs.push(Opcode::ILoad(local.index)),
            _ => unreachable!(),
        };
        bytecode
    }

    fn generate_binop(&mut self, binop: BinaryOp) -> Bytecode {
        let mut bytecode = Bytecode::make(vec![]);
        match binop {
            BinaryOp::Plus(type_) => match type_.unwrap() {
                Type::Integer => bytecode.instrs.push(Opcode::IAdd),
                _ => unreachable!(),
            },
        }
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
