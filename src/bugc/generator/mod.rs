use std::collections::HashMap;

use bug::{
  bytecode::{ByteCodeStream, Opcode},
  DefinedFn, Object, Pool, PoolEntry, Program,
};

use super::ast::*;

#[derive(Debug)]
struct Scope {
  bytecode: ByteCodeStream,
}

impl Scope {
  fn new() -> Self {
    Self { bytecode: ByteCodeStream::empty() }
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
    for statement in &function.body {
      self.emit_statement(statement);
    }

    self.current_scope.bytecode.push(Opcode::Return);
    let defined_fn = DefinedFn { arity: 0, code: self.current_scope.bytecode.clone(), max_locals: 0 };
    self.functions.insert(function.get_name().to_owned(), defined_fn);
  }

  fn emit_statement_expression(&mut self, expression: &StatementExpression) {
    match expression {
      StatementExpression::Call(call) => self.emit_expression_call(call),
      StatementExpression::Literal(literal) => self.emit_expression_literal(literal),
    };
  }

  fn emit_expression_call(&mut self, call: &ExpressionCall) {
    self.current_scope.bytecode.push(Opcode::Invoke(call.get_name().to_owned()));
  }

  fn emit_expression_literal(&mut self, literal: &ExpressionLiteral) {
    match literal {
      ExpressionLiteral::String(string) => self.emit_literal_string(string.get_data()),
      ExpressionLiteral::Integer(_) => todo!(),
    }
  }

  fn emit_literal_string(&mut self, string: &str) {
    let pool_entry = PoolEntry::Object(Object::String(string.to_owned()));
    let index = self.constant_pool.append(pool_entry);
    self.current_scope.bytecode.push(Opcode::Ldc(index));
  }
}
