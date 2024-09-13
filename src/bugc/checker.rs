use crate::ast::*;
use crate::highlighter::highlight_error;
use crate::span::Span;
use bug::stdlib::NativeFn;
use bug::{FunctionPrototype, Type};
use std::collections::HashMap;

pub struct Checker<'a> {
  file_path: &'a str,
  file_content: &'a str,
  ast: &'a Ast,
  ctx: Context,
  diagnostics: Diagnostics,
}

impl<'a> Checker<'a> {
  pub fn new(file_path: &'a str, file_content: &'a str, ast: &'a Ast, natives: HashMap<String, NativeFn>) -> Self {
    Self { file_path, file_content, ast, ctx: Context::new(natives), diagnostics: Diagnostics::new() }
  }

  pub fn check(&mut self) -> &Diagnostics {
    for statement in self.ast {
      self.check_statement(statement);
    }
    &self.diagnostics
  }

  fn check_statement(&mut self, statement: &Statement) {
    match statement {
      Statement::Function(f) => self.check_statement_function(f),
      Statement::Expression(expression) => self.check_statement_expression(expression),
    };
  }

  fn check_statement_function(&mut self, f: &StatementFunction) {
    let name = f.identifier.label.clone();
    if let Some(_) = self.ctx.lookup_locally(&name) {
      self.error_name_already_used(&name, &f.identifier.span);
      return;
    }
    let param_types: Vec<Type> = f.parameters.parameters.iter().map(|p| p.typ.clone()).collect();
    let prototype = FunctionPrototype::new(param_types.len(), f.return_type.clone(), param_types);
    self.ctx.declare(name.clone(), Symbol::Function(prototype));
    self.ctx.enter_scope(ScopeType::Function);
    for parameter in &f.parameters.parameters {
      if let Some(_) = self.ctx.lookup_locally(&parameter.identifier.label) {
        self.error_name_already_used(&parameter.identifier.label, &parameter.span);
        return;
      }
      self.ctx.declare(parameter.identifier.label.clone(), Symbol::Variable(Variable { typ: parameter.typ.clone() }))
    }
    for statement in &f.body.statements {
      self.check_statement(statement);
    }
    let (returned_type, span) = self.ctx.pop().unwrap_or((Type::Void, f.body.span.clone()));
    if returned_type != f.return_type {
      self.error_return_type(name, &span, f.return_type.clone(), returned_type);
    }
    self.ctx.leave_scope();
  }

  fn check_statement_expression(&mut self, expression: &StatementExpression) {
    match expression {
      StatementExpression::Call(call) => self.check_expression_call(call),
      StatementExpression::Binary(binary) => self.check_expression_binary(binary),
      StatementExpression::Literal(literal) => self.check_expression_literal(literal),
      StatementExpression::Identifier(identifier) => self.check_expression_identifier(identifier),
    };
  }

  fn check_expression_identifier(&mut self, identifier: &ExpressionIdentifier) {
    let symbol = match self.ctx.lookup(&identifier.name) {
      Some(symbol) => symbol,
      None => return self.error_name_not_declared(&identifier.name, &identifier.span),
    };
    match symbol {
      Symbol::Variable(v) => self.ctx.push(v.typ.clone(), identifier.span.clone()),
      Symbol::Function(_) => unimplemented!(),
    };
  }

  fn check_expression_call(&mut self, call: &ExpressionCall) {
    let callee = self.ctx.lookup(&call.identifier.label);
    if callee.is_none() {
      self.error_name_not_declared(&call.identifier.label, &call.identifier.span);
      return;
    }
    let callee = match callee.unwrap() {
      Symbol::Function(f) => f.clone(),
      _ => todo!("{} is not callable", call.identifier.label),
    };
    if self.ctx.stack_depth() < callee.arity {
      self.ctx.pop_many(self.ctx.stack_depth());
      self.error_missing_args(call.identifier.label.clone(), &call.span);
      return;
    }
    let provided_args = self.ctx.pop_many(callee.arity);
    for ((provided_type, span), expected_type) in provided_args.into_iter().zip(callee.parameters_types) {
      if provided_type != expected_type {
        self.error_arg_type_no_match(expected_type, provided_type, &span);
      }
    }
    if callee.return_type != Type::Void {
      self.ctx.push(callee.return_type, call.span.clone());
    }
  }

  fn check_expression_binary(&mut self, binary: &ExpressionBinary) {
    if self.ctx.stack_depth() < 2 {
      self.error_miss_binexpr_args(&binary.operator, &binary.span);
      return;
    }
    let (rhs_type, _) = self.ctx.pop().unwrap();
    let (lhs_type, lhs_span) = self.ctx.pop().unwrap();
    let span = Span::new(lhs_span.line, lhs_span.column, lhs_span.start, binary.span.end);
    if lhs_type != rhs_type {
      self.error_binexpr_types_no_match(&binary.operator, lhs_type, rhs_type, &span);
      return;
    }
    match binary.operator {
      BinaryOperator::Plus => self.check_binary_plus(lhs_type, rhs_type, span),
      BinaryOperator::Minus => self.check_binary_minus(lhs_type, rhs_type, span),
    }
  }

  fn check_binary_plus(&mut self, lhs: Type, rhs: Type, span: Span) {
    match lhs {
      Type::Integer => self.ctx.push(Type::Integer, span),
      Type::String => self.ctx.push(Type::String, span),
      _ => self.error_binexpr_types_no_match(&BinaryOperator::Plus, lhs, rhs, &span),
    };
  }

  fn check_binary_minus(&mut self, lhs: Type, rhs: Type, span: Span) {
    match lhs {
      Type::Integer => self.ctx.push(Type::Integer, span),
      _ => self.error_binexpr_types_no_match(&BinaryOperator::Minus, lhs, rhs, &span),
    };
  }

  fn check_expression_literal(&mut self, literal: &ExpressionLiteral) {
    match literal {
      ExpressionLiteral::String(string) => self.ctx.push(Type::String, string.span.clone()),
      ExpressionLiteral::Integer(integer) => self.ctx.push(Type::Integer, integer.span.clone()),
    };
  }
}

enum Symbol {
  Function(FunctionPrototype),
  Variable(Variable),
}

struct Variable {
  typ: Type,
}

enum ScopeType {
  Global,
  Function,
}

struct Scope {
  typ: ScopeType,
  table: HashMap<String, Symbol>,
  stack: Vec<(Type, Span)>,
}

impl Scope {
  fn new(typ: ScopeType) -> Self {
    Self { typ, table: HashMap::new(), stack: vec![] }
  }

  fn from(typ: ScopeType, table: HashMap<String, Symbol>) -> Self {
    Self { typ, table, stack: vec![] }
  }
}

struct Context {
  scope_pointer: usize,
  scopes: Vec<Scope>,
}

impl Context {
  fn new(natives: HashMap<String, NativeFn>) -> Self {
    let mut table: HashMap<String, Symbol> = HashMap::new();
    for (name, f) in natives {
      table.insert(name, Symbol::Function(f.prototype));
    }
    Self { scope_pointer: 0, scopes: vec![Scope::from(ScopeType::Global, table)] }
  }

  fn lookup(&self, name: &str) -> Option<&Symbol> {
    for scope in self.scopes.iter().rev() {
      if let Some(x) = scope.table.get(name) {
        return Some(x);
      }
    }
    None
  }

  fn lookup_locally(&self, name: &str) -> Option<&Symbol> {
    self.scopes.get(self.scope_pointer).unwrap().table.get(name)
  }

  fn declare(&mut self, key: String, value: Symbol) {
    self.scopes[self.scope_pointer].table.insert(key, value);
  }

  fn enter_scope(&mut self, typ: ScopeType) {
    self.scope_pointer += 1;
    self.scopes.push(Scope::new(typ));
  }

  fn leave_scope(&mut self) {
    self.scope_pointer -= 1;
    self.scopes.pop();
  }

  fn stack_depth(&self) -> usize {
    self.scopes[self.scope_pointer].stack.len()
  }

  fn pop(&mut self) -> Option<(Type, Span)> {
    self.scopes[self.scope_pointer].stack.pop().clone()
  }

  fn push(&mut self, typ: Type, span: Span) {
    self.scopes[self.scope_pointer].stack.push((typ, span));
  }

  // @NOTE: ensure that the stack has the amount of items to pop otherwise the program will panic
  fn pop_many(&mut self, count: usize) -> Vec<(Type, Span)> {
    let mut types: Vec<(Type, Span)> = vec![];
    for _ in 0..count {
      types.push(self.scopes[self.scope_pointer].stack.pop().unwrap())
    }
    types
  }
}

pub struct Diagnostics {
  diagnostics: Vec<String>,
}

impl core::fmt::Display for Diagnostics {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for diagnostic in &self.diagnostics {
      write!(f, "{}", diagnostic)?;
    }
    Ok(())
  }
}

impl Diagnostics {
  fn new() -> Self {
    Self { diagnostics: vec![] }
  }
}

impl<'a> Checker<'a> {
  fn error_name_already_used(&mut self, name: &str, span: &Span) {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(&format!("Name `{}` is already used", name));
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    self.diagnostics.diagnostics.push(error);
  }

  fn error_return_type(&mut self, name: String, span: &Span, expected: Type, provided: Type) {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(&format!("Function `{}` returns `{}` but got `{}` ", name, expected, provided));
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    self.diagnostics.diagnostics.push(error);
  }

  fn error_name_not_declared(&mut self, name: &str, span: &Span) {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(&format!("Name `{}` is not declared", name));
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    self.diagnostics.diagnostics.push(error);
  }

  fn error_missing_args(&mut self, name: String, span: &Span) {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(&format!("Missing arguments calling `{}`", name));
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    self.diagnostics.diagnostics.push(error);
  }

  fn error_arg_type_no_match(&mut self, expected: Type, provided: Type, span: &Span) {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(&format!("Arguement of type `{}` is not assignable to parameter of type `{}`", provided, expected));
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    self.diagnostics.diagnostics.push(error);
  }

  fn error_binexpr_types_no_match(&mut self, op: &BinaryOperator, lhs_type: Type, rhs_type: Type, span: &Span) {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(&format!("Operator `{}` doesn't apply to types `{}` and `{}`", op, lhs_type, rhs_type));
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    self.diagnostics.diagnostics.push(error);
  }

  fn error_miss_binexpr_args(&mut self, op: &BinaryOperator, span: &Span) {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(&format!("Missing arguments for `{}` operator", op));
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    self.diagnostics.diagnostics.push(error);
  }

  fn error_header(&self, span: &Span) -> String {
    format!(
      "\x1b[38;5;4m{}\x1b[0m:\x1b[38;5;5m{}\x1b[0m:\x1b[38;5;5m{}\x1b[0m\x1b[1;31m ERROR\x1b[0m ",
      self.file_path, span.line, span.column
    )
  }
}
