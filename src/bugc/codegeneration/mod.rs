use std::collections::HashMap;

use crate::ast::*;
use bug::bytecode::{Bytecode, Opcode, PushOperand};
use bug::{DefinedFn, Object, Pool, PoolEntry, Program, Type};

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

    fn generate_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::If(block) => self.generate_if_statement(block),
            Statement::Expression(expr) => self.generate_expression(expr),
            Statement::FunctionDeclaration(fn_decl) => self.generate_function_declaration(fn_decl),
            Statement::VariableDeclaration(var_decl) => self.generate_variable_decl(var_decl),
            Statement::Assignment(target) => self.generate_assignment(target),
        }
    }

    fn generate_assignment(&mut self, target: Option<String>) {
        self.context.bytecode.push(Opcode::LStore(
            self.context.locals.get(&target.unwrap()).unwrap().index,
        ));
    }

    fn generate_variable_decl(&mut self, var_decl: VariableDeclaration) {
        self.context.locals.insert(
            var_decl.name,
            Local::make(self.context.locals.len(), var_decl.type_),
        );
    }

    fn generate_if_statement(&mut self, block: BlockStatement) {
        // Adding "No operation" as placeholder to substitute later with a "JumpIfFalse" op
        let nop_index = self.context.bytecode.instrs.len();
        self.context.bytecode.push(Opcode::Nop);

        for stmt in block {
            self.generate_statement(stmt);
        }
        let after_if_block = self.context.bytecode.instrs.len();
        self.context.bytecode.instrs[nop_index] = Opcode::JumpIfFalse(after_if_block);
    }

    fn generate_function_declaration(&mut self, fn_decl: FunctionDeclaration) {
        self.context.reset();

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
                max_locals: self.context.locals.len(),
            },
        );
    }

    fn generate_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Literal(literal) => self.generate_literal(literal),
            Expression::FunctionCall(fn_name) => self.generate_function_call(fn_name),
            Expression::BinaryOp(binop) => self.generate_binop(binop),
            Expression::Identifier(ident) => self.generate_identifier(ident),
            Expression::Return(type_) => self.generate_return_expression(type_.unwrap()),
        }
    }

    fn generate_identifier(&mut self, ident: String) {
        let local = self
            .context
            .locals
            .get(&ident)
            .expect(&format!("Expected '{}' to a local", &ident));
        match local.type_ {
            Type::Integer | Type::Boolean | Type::String => {
                self.context.bytecode.push(Opcode::LLoad(local.index))
            }
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
            Literal::Int(x) => self
                .context
                .bytecode
                .push(Opcode::Push(PushOperand::Integer(x))),
            Literal::Boolean(x) => self
                .context
                .bytecode
                .push(Opcode::Push(PushOperand::Boolean(x))),
            Literal::String(x) => self.context.bytecode.push(Opcode::Ldc(
                self.pool.append(PoolEntry::Object(Object::String(x))),
            )),
        };
    }

    fn generate_return_expression(&mut self, type_: Type) {
        match type_ {
            Type::Void => self.context.bytecode.push(Opcode::Return),
            Type::Integer => self.context.bytecode.push(Opcode::IReturn),
            _ => unimplemented!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bug::Type;

    #[test]
    fn enaure_hello_world_is_well_generated() {
        let ast = vec![Statement::FunctionDeclaration(FunctionDeclaration {
            name: "main".to_string(),
            params: vec![],
            return_type: Type::Void,
            body: vec![
                Statement::Expression(Expression::Literal(Literal::String(
                    "Hello, world!".to_string(),
                ))),
                Statement::Expression(Expression::FunctionCall("write".to_string())),
            ],
        })];

        let mut generator = CodeGenerator::make();
        let program = generator.gen(ast);

        assert!(program.fns.contains_key("main"));
        assert!(program
            .pool
            .entries
            .contains(&PoolEntry::Object(Object::String(
                "Hello, world!".to_string()
            ))));

        let main_instructions = program.fns.get("main").unwrap().code.instrs.clone();

        match &main_instructions[0] {
            Opcode::Ldc(_) => {}
            x => panic!("Unexpected instruction {}", x),
        }

        match &main_instructions[1] {
            Opcode::Invoke(_) => {}
            x => panic!("Unexpected instruction {}", x),
        }

        match &main_instructions[2] {
            Opcode::Return => {}
            x => panic!("Unexpected instruction {}", x),
        }
    }
}
