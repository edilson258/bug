use super::ast::*;
use bug::bytecode::{ByteCodeStream, Opcode, PushOperand};
use bug::{DefinedFn, Object, Pool, Program};
use std::collections::HashMap;

#[derive(Debug)]
struct Scope {
  bytecode: ByteCodeStream,
}

impl Scope {
  fn new() -> Self {
    Self { bytecode: ByteCodeStream::empty() }
  }

  fn reset(&mut self) {
    self.bytecode.clear();
  }

  fn push_op(&mut self, op: Opcode) {
    self.bytecode.push(op);
  }
}

#[derive(Debug)]
pub struct CodeGenerator<'a> {
  ast: &'a Ast,
  current_scope: Scope,
  constant_pool: Pool,
  functions: HashMap<String, DefinedFn>,
}

impl<'a> CodeGenerator<'a> {
  pub fn new(ast: &'a Ast) -> Self {
    Self { ast, current_scope: Scope::new(), functions: HashMap::new(), constant_pool: Pool::make() }
  }

  pub fn emit(&mut self) -> Program {
    for statement in self.ast {
      self.emit_statement(statement);
    }
    Program { pool: self.constant_pool.clone(), fns: self.functions.clone() }
  }

  fn emit_statement(&mut self, statement: &Statement) {
    match statement {
      Statement::Function(function) => self.emit_statement_function(function),
      Statement::Expression(expression) => self.emit_statement_expression(expression),
    };
  }

  fn emit_statement_function(&mut self, function: &StatementFunction) {
    self.current_scope.reset();

    for statement in &function.body {
      self.emit_statement(statement);
    }
    self.current_scope.push_op(Opcode::Return);

    let defined_fn = DefinedFn {
      start_line: function.identifier.span.line,
      arity: 0,
      code: self.current_scope.bytecode.clone(),
      max_locals: 0,
    };
    self.functions.insert(function.get_name().to_owned(), defined_fn);
  }

  fn emit_statement_expression(&mut self, expression: &StatementExpression) {
    match expression {
      StatementExpression::Call(call) => self.emit_expression_call(call),
      StatementExpression::Literal(literal) => self.emit_expression_literal(literal),
      StatementExpression::Binary(binary_expression) => self.emit_expression_binary(binary_expression),
    };
  }

  fn emit_expression_binary(&mut self, binop: &ExpressionBinary) {
    match binop.operator {
      BinaryOperator::Add => self.current_scope.push_op(Opcode::Add),
    };
  }

  fn emit_expression_call(&mut self, call: &ExpressionCall) {
    self.current_scope.bytecode.push(Opcode::Invoke(call.get_name().to_owned()));
  }

  fn emit_expression_literal(&mut self, literal: &ExpressionLiteral) {
    match literal {
      ExpressionLiteral::String(string) => self.emit_literal_string(string.get_data()),
      ExpressionLiteral::Number(number) => self.emit_literal_number(number.get_data()),
    }
  }

  fn emit_literal_string(&mut self, string: &str) {
    let o = Object::String(string.to_owned());
    let index = self.constant_pool.append(o);

    self.current_scope.push_op(Opcode::Ldc(index));
  }

  fn emit_literal_number(&mut self, number: &f32) {
    self.current_scope.push_op(Opcode::Push(PushOperand::Number(number.to_owned())));
  }
}
