use super::ast::*;
use bug::*;
use bytecode::{ByteCodeStream, Opcode};
use std::collections::HashMap;

struct Context {
    code: ByteCodeStream,
    locals: HashMap<String, usize>,
}

impl Context {
    fn new() -> Self {
        Context { code: ByteCodeStream::empty(), locals: HashMap::new() }
    }

    fn push(&mut self, op: Opcode) {
        self.code.push(op);
    }

    fn reset(&mut self) {
        self.code.code.clear();
        self.locals.clear();
    }
}

pub struct CodeGenerator {
    program: Program,
    context: Context,
}

impl CodeGenerator {
    pub fn setup() -> Self {
        Self { program: Program::new(), context: Context::new() }
    }

    pub fn emit(&mut self, ast: Ast) -> Program {
        for statement in ast {
            self.emit_statement(statement);
        }
        self.program.clone()
    }

    fn emit_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Function(f) => self.emit_statement_function(f),
            Statement::Expression(e) => self.emit_statement_expression(e),
        };
    }

    fn emit_statement_function(&mut self, f: StatementFunction) {
        self.context.reset();
        let name = f.identifier.label;
        let arity = f.parameters.parameters.len();
        for (index, parameter) in f.parameters.parameters.into_iter().enumerate() {
            self.context.locals.insert(parameter.identifier.label, index);
        }
        for statement in f.body.statements {
            self.emit_statement(statement);
        }
        self.context.push(Opcode::RETURN);
        let max_locals = self.context.locals.len();
        let code = self.context.code.clone();
        self.program.fns.insert(name, DefinedFn::new(0, arity, code, max_locals));
    }

    fn emit_statement_expression(&mut self, expression: StatementExpression) {
        match expression {
            StatementExpression::Call(call) => self.emit_expression_call(call),
            StatementExpression::Binary(binary) => self.emit_expression_binary(binary),
            StatementExpression::Literal(literal) => self.emit_expression_literal(literal),
            StatementExpression::Identifier(identifier) => self.emit_expression_identifier(identifier),
            StatementExpression::Ternary(ternary) => self.emit_expression_ternary(ternary),
        };
    }

    // fn max(int l, int r) int -> l r > ? l : r;

    fn emit_expression_ternary(&mut self, ternary: ExpressionTernary) {
        let before_cond_offset = self.context.code.code.len();
        self.context.code.push(Opcode::NOP);
        self.emit_statement_expression(*ternary.consequence);
        let after_consq_offset = self.context.code.code.len();
        self.context.code.push(Opcode::NOP);
        self.emit_statement_expression(*ternary.alternative);
        let after_alt_offset = self.context.code.code.len();
        self.context.code.push_at(Opcode::JUMPNOTIF(after_consq_offset + 1), before_cond_offset);
        self.context.code.push_at(Opcode::JUMP(after_alt_offset), after_consq_offset);
    }

    fn emit_expression_call(&mut self, call: ExpressionCall) {
        self.context.push(Opcode::INVOKE(call.identifier.label));
    }

    fn emit_expression_binary(&mut self, binary: ExpressionBinary) {
        let operands_types = binary.operands_types.unwrap();
        match binary.operator {
            BinaryOperator::Plus => self.emit_binary_plus(operands_types),
            BinaryOperator::Minus => self.emit_binary_minus(operands_types),
            BinaryOperator::GratherThan => self.emit_binary_gt(operands_types),
        };
    }

    fn emit_binary_plus(&mut self, operands_types: Type) {
        match operands_types {
            Type::Integer => self.context.push(Opcode::IADD),
            Type::String => todo!(),
            _ => unreachable!(),
        }
    }

    fn emit_binary_minus(&mut self, operands_types: Type) {
        match operands_types {
            Type::Integer => todo!(),
            _ => unreachable!(),
        }
    }

    fn emit_binary_gt(&mut self, operands_types: Type) {
        match operands_types {
            Type::Integer => self.context.push(Opcode::ICMPGT),
            _ => unreachable!(),
        }
    }

    fn emit_expression_literal(&mut self, literal: ExpressionLiteral) {
        match literal {
            ExpressionLiteral::String(string) => self.emit_literal_string(string),
            ExpressionLiteral::Integer(integer) => self.emit_literal_integer(integer),
        };
    }

    fn emit_literal_integer(&mut self, integer: LiteralInteger) {
        self.context.push(Opcode::IPUSH(integer.inner));
    }

    fn emit_literal_string(&mut self, string: LiteralString) {
        let index = self.program.pool.append(Object::String(string.inner));
        self.context.push(Opcode::LDC(index));
    }

    fn emit_expression_identifier(&mut self, identifier: Identifier) {
        if let Some(index) = self.context.locals.get(&identifier.label) {
            self.context.push(Opcode::LLOAD(*index));
        } else {
            unreachable!()
        }
    }
}
