use std::collections::HashMap;

use crate::ast::*;

use spider_vm::bytecode::{Bytecode, Opcode};
use spider_vm::object::Object;
use spider_vm::pool::{Pool, PoolEntry};
use spider_vm::program::{DefinedFn, Program};
use spider_vm::stdlib::Type;

struct Context {
    bytecode: Bytecode,
    locals: HashMap<String, Local>,
}

impl Context {
    pub fn make() -> Self {
        Self {
            bytecode: Bytecode::make(vec![]),
            locals: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.bytecode.instrs.clear();
        self.locals.clear();
    }
}

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
    context: Context,
}

impl CodeGenerator {
    pub fn make() -> Self {
        Self {
            pool: Pool::make(),
            fns: HashMap::new(),
            context: Context::make(),
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

    fn generate_statement(&mut self, stmt: Statment) {
        match stmt {
            Statment::Expression(expr) => self.generate_expression(expr),
            Statment::FunctionDeclaration(fn_decl) => self.generate_function_declaration(fn_decl),
            Statment::If(block) => self.generate_if_statement(block),
            Statment::Return(type_) => self.generate_return_statement(type_.unwrap()),
        }
    }

    fn generate_if_statement(&mut self, block: BlockStatment) {
        // Adding "No operation" as placeholder to substitute later with a "JumpIfFalse" op
        let nop_index = self.context.bytecode.instrs.len();
        self.context.bytecode.push(Opcode::Nop);

        for stmt in block {
            self.generate_statement(stmt);
        }
        let after_if_block = self.context.bytecode.instrs.len();
        self.context.bytecode.instrs[nop_index] = Opcode::JumpIfFalse(after_if_block);
    }

    fn generate_return_statement(&mut self, type_: Type) {
        match type_ {
            Type::Void => self.context.bytecode.push(Opcode::Return),
            Type::Integer => self.context.bytecode.push(Opcode::IReturn),
            _ => unimplemented!(),
        };
    }

    fn generate_function_declaration(&mut self, fn_decl: FunctionDeclaration) {
        let arity = fn_decl.params.len();
        for (i, p) in fn_decl.params.into_iter().enumerate() {
            self.context.locals.insert(p.name, Local::make(i, p.type_));
        }
        for stmt in fn_decl.body {
            self.generate_statement(stmt);
        }

        match fn_decl.return_type {
            Type::Integer => self.context.bytecode.push(Opcode::IReturn),
            _ => self.context.bytecode.push(Opcode::Return),
        }

        self.fns.insert(
            fn_decl.name,
            DefinedFn {
                arity,
                code: self.context.bytecode.clone(),
                max_locals: arity,
            },
        );
        self.context.reset();
    }

    fn generate_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Literal(literal) => self.generate_literal(literal),
            Expression::FunctionCall(fn_name) => self.generate_function_call(fn_name),
            Expression::BinaryOp(binop) => self.generate_binop(binop),
            Expression::Identifier(ident) => self.generate_identifier(ident),
        }
    }

    fn generate_identifier(&mut self, ident: String) {
        let local = self
            .context
            .locals
            .get(&ident)
            .expect(&format!("Expected '{}' to a local", &ident));
        match local.type_ {
            Type::Integer => self.context.bytecode.push(Opcode::ILoad(local.index)),
            _ => unreachable!(),
        };
    }

    fn generate_binop(&mut self, binop: BinaryOp) {
        match binop {
            BinaryOp::Plus(type_) => match type_.unwrap() {
                Type::Integer => self.context.bytecode.push(Opcode::IAdd),
                _ => unreachable!(),
            },
            BinaryOp::GratherThan(type_) => match type_.unwrap() {
                Type::Integer => self.context.bytecode.push(Opcode::ICmpGT),
                _ => unreachable!(),
            },
        }
    }

    fn generate_function_call(&mut self, fn_name: String) {
        self.context.bytecode.push(Opcode::Invoke(fn_name));
    }

    fn generate_literal(&mut self, literal: Literal) {
        match literal {
            Literal::Int(x) => self.context.bytecode.push(Opcode::Bipush(x)),
            Literal::String(x) => self.context.bytecode.push(Opcode::Ldc(
                self.pool.append(PoolEntry::Object(Object::String(x))),
            )),
        };
    }
}
