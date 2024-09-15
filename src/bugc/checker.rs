use crate::ast::*;
use crate::highlighter::highlight_error;
use crate::span::Span;
use bug::stdlib::NativeFn;
use bug::{FunctionPrototype, Type};
use std::collections::HashMap;

pub struct Checker<'a> {
  file_path: &'a str,
  file_content: &'a str,
  ctx: Context,
  diagnostics: Diagnostics,
}

impl<'a> Checker<'a> {
  pub fn new(file_path: &'a str, file_content: &'a str, natives: HashMap<String, NativeFn>) -> Self {
    Self { file_path, file_content, ctx: Context::new(natives), diagnostics: Diagnostics::new() }
  }

  pub fn check(&mut self, ast: &'a mut Ast) -> Option<&Diagnostics> {
    for statement in ast {
      if let Some(err) = self.check_statement(statement).err() {
        self.diagnostics.diagnostics.push(err);
      }
    }
    if self.diagnostics.diagnostics.is_empty() {
      None
    } else {
      Some(&self.diagnostics)
    }
  }

  fn check_statement(&mut self, statement: &mut Statement) -> Result<(), String> {
    match statement {
      Statement::Function(f) => Ok(self.check_statement_function(f)?),
      Statement::Expression(expression) => Ok(self.check_statement_expression(expression)?),
    }
  }

  fn check_statement_function(&mut self, f: &mut StatementFunction) -> Result<(), String> {
    let name = f.identifier.label.clone();
    if let Some(_) = self.ctx.lookup_locally(&name) {
      return Err(self.error_name_already_used(&name, &f.identifier.span));
    }
    let param_types: Vec<Type> = f.parameters.parameters.iter().map(|p| p.typ.clone()).collect();
    let prototype = FunctionPrototype::new(param_types.len(), f.return_type.clone(), param_types);
    self.ctx.declare(name.clone(), Symbol::Function(prototype));
    self.ctx.enter_scope(ScopeType::Function);
    for parameter in &f.parameters.parameters {
      if let Some(_) = self.ctx.lookup_locally(&parameter.identifier.label) {
        return Err(self.error_name_already_used(&parameter.identifier.label, &parameter.identifier.span));
      }
      self.ctx.declare(parameter.identifier.label.clone(), Symbol::Variable(Variable { typ: parameter.typ.clone() }))
    }
    for statement in &mut f.body.statements {
      self.check_statement(statement)?;
    }
    let (returned_type, span) = self.ctx.pop().unwrap_or((Type::Void, f.body.span.clone()));
    if returned_type != f.return_type {
      let err = self.error_return_type(&name, &span, &f.return_type, &returned_type);
      self.diagnostics.diagnostics.push(err);
    }
    self.ctx.leave_scope();
    Ok(())
  }

  fn check_statement_expression(&mut self, expression: &mut StatementExpression) -> Result<(), String> {
    match expression {
      StatementExpression::Call(call) => Ok(self.check_expression_call(call)?),
      StatementExpression::Binary(binary) => Ok(self.check_expression_binary(binary)?),
      StatementExpression::Literal(literal) => Ok(self.check_expression_literal(literal)?),
      StatementExpression::Identifier(identifier) => Ok(self.check_expression_identifier(identifier)?),
      StatementExpression::Ternary(ternary) => self.check_expression_ternary(ternary),
    }
  }

  fn check_expression_ternary(&mut self, ternary: &mut ExpressionTernary) -> Result<(), String> {
    if self.ctx.stack_depth() < 1 {
      return Err(self.error_miss_ternary_cond(&ternary.span));
    }
    let (cond_typ, cond_span) = self.ctx.pop().unwrap();
    if Type::Boolean != cond_typ {
      return Err(self.error_unexpected_type(&Type::Boolean, &cond_typ, &cond_span));
    }
    self.check_statement_expression(&mut ternary.consequence)?;
    let (consq_type, _) = self.ctx.pop().unwrap_or((Type::Void, ternary.span.clone()));
    self.check_statement_expression(&mut ternary.alternative)?;
    let (alt_type, _) = self.ctx.pop().unwrap_or((Type::Void, ternary.span.clone()));
    if consq_type != alt_type {
      return Err(self.error_ternary_arms_no_match(&consq_type, &alt_type, &ternary.span));
    }
    self.ctx.push(consq_type, ternary.span.clone());
    Ok(())
  }

  fn check_expression_identifier(&mut self, identifier: &ExpressionIdentifier) -> Result<(), String> {
    let symbol = match self.ctx.lookup(&identifier.name) {
      Some(symbol) => symbol,
      None => {
        return Err(self.error_name_not_declared(&identifier.name, &identifier.span));
      }
    };
    match symbol {
      Symbol::Variable(v) => Ok(self.ctx.push(v.typ.clone(), identifier.span.clone())),
      Symbol::Function(_) => unimplemented!(),
    }
  }

  fn check_expression_call(&mut self, call: &ExpressionCall) -> Result<(), String> {
    let callee = self.ctx.lookup(&call.identifier.label);
    if callee.is_none() {
      return Err(self.error_name_not_declared(&call.identifier.label, &call.identifier.span));
    }
    let callee = match callee.unwrap() {
      Symbol::Function(f) => f.clone(),
      _ => todo!("{} is not callable", call.identifier.label),
    };
    if self.ctx.stack_depth() < callee.arity {
      self.ctx.pop_many(self.ctx.stack_depth());
      return Err(self.error_missing_args(&call.identifier.label, &call.span));
    }
    let provided_args = self.ctx.pop_many(callee.arity);
    for ((provided_type, span), expected_type) in provided_args.into_iter().zip(callee.parameters_types) {
      if provided_type != expected_type {
        let err = self.error_arg_type_no_match(&expected_type, &provided_type, &span);
        self.diagnostics.diagnostics.push(err);
      }
    }
    self.ctx.push(callee.return_type, call.span.clone());
    Ok(())
  }

  fn check_expression_binary(&mut self, binary: &mut ExpressionBinary) -> Result<(), String> {
    if self.ctx.stack_depth() < 2 {
      return Err(self.error_miss_binexpr_args(&binary.operator, &binary.span));
    }
    let (rhs_type, _) = self.ctx.pop().unwrap();
    let (lhs_type, lhs_span) = self.ctx.pop().unwrap();
    let span = Span::new(lhs_span.line, lhs_span.column, lhs_span.start, binary.span.end);
    if lhs_type != rhs_type {
      return Err(self.error_binexpr_types_no_match(&binary.operator, &lhs_type, &rhs_type, &span));
    }
    binary.operands_types = Some(lhs_type.clone());
    match binary.operator {
      BinaryOperator::Plus => Ok(self.check_binary_plus(lhs_type, rhs_type, span)?),
      BinaryOperator::Minus => Ok(self.check_binary_minus(lhs_type, rhs_type, span)?),
      BinaryOperator::GratherThan => Ok(self.check_binary_gt(lhs_type, rhs_type, span)?),
    }
  }

  fn check_binary_plus(&mut self, lhs: Type, _rhs: Type, span: Span) -> Result<(), String> {
    match lhs {
      Type::Integer => Ok(self.ctx.push(Type::Integer, span)),
      Type::String => Ok(self.ctx.push(Type::String, span)),
      _ => Err(self.error_invalid_operator_operands(&BinaryOperator::Plus, &lhs, &span)),
    }
  }

  fn check_binary_minus(&mut self, lhs: Type, _rhs: Type, span: Span) -> Result<(), String> {
    match lhs {
      Type::Integer => Ok(self.ctx.push(Type::Integer, span)),
      _ => Err(self.error_invalid_operator_operands(&BinaryOperator::Minus, &lhs, &span)),
    }
  }

  fn check_binary_gt(&mut self, lhs: Type, _rhs: Type, span: Span) -> Result<(), String> {
    match lhs {
      Type::Integer => Ok(self.ctx.push(Type::Boolean, span)),
      _ => Err(self.error_invalid_operator_operands(&BinaryOperator::GratherThan, &lhs, &span)),
    }
  }

  fn check_expression_literal(&mut self, literal: &ExpressionLiteral) -> Result<(), String> {
    match literal {
      ExpressionLiteral::String(string) => Ok(self.ctx.push(Type::String, string.span.clone())),
      ExpressionLiteral::Integer(integer) => Ok(self.ctx.push(Type::Integer, integer.span.clone())),
    }
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
  _typ: ScopeType,
  table: HashMap<String, Symbol>,
  stack: Vec<(Type, Span)>,
}

impl Scope {
  fn new(typ: ScopeType) -> Self {
    Self { _typ: typ, table: HashMap::new(), stack: vec![] }
  }

  fn from(typ: ScopeType, table: HashMap<String, Symbol>) -> Self {
    Self { _typ: typ, table, stack: vec![] }
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
  fn error_name_already_used(&mut self, name: &str, span: &Span) -> String {
    self.error(&format!("Name `{}` is already used", name), span)
  }

  fn error_return_type(&mut self, name: &str, span: &Span, expected: &Type, provided: &Type) -> String {
    self.error(&format!("Function `{}` returns `{}` but got `{}` ", name, expected, provided), span)
  }

  fn error_name_not_declared(&mut self, name: &str, span: &Span) -> String {
    self.error(&format!("Name `{}` is not declared", name), span)
  }

  fn error_missing_args(&mut self, name: &str, span: &Span) -> String {
    self.error(&format!("Missing arguments calling `{}`", name), span)
  }

  fn error_arg_type_no_match(&mut self, expected: &Type, provided: &Type, span: &Span) -> String {
    self.error(&format!("Arguement of type `{}` is not assignable to parameter of type `{}`", provided, expected), span)
  }

  fn error_binexpr_types_no_match(
    &mut self,
    op: &BinaryOperator,
    lhs_type: &Type,
    rhs_type: &Type,
    span: &Span,
  ) -> String {
    self.error(&format!("Operator `{}` doesn't apply to types `{}` and `{}`", op, lhs_type, rhs_type), span)
  }

  fn error_miss_binexpr_args(&mut self, op: &BinaryOperator, span: &Span) -> String {
    self.error(&format!("Missing arguments for `{}` operator", op), span)
  }

  fn error_invalid_operator_operands(&mut self, op: &BinaryOperator, typ: &Type, span: &Span) -> String {
    self.error(&format!("Operator `{}` doesn't apply to values of type `{}`", op, typ), span)
  }

  fn error_miss_ternary_cond(&mut self, span: &Span) -> String {
    self.error("Missing condition for the ternary operator", span)
  }

  fn error_unexpected_type(&mut self, expected: &Type, provided: &Type, span: &Span) -> String {
    self.error(&format!("Expecting value of type `{}` but got `{}`", expected, provided), span)
  }

  fn error_ternary_arms_no_match(&mut self, consq_type: &Type, alt_type: &Type, span: &Span) -> String {
    let mut message = String::new();
    message.push_str("`?` operator arms have different types\n");
    message.push_str(&format!("\tFirst arm returns `{}` and alternative returns `{}`", consq_type, alt_type));
    self.error(&message, span)
  }

  fn error(&mut self, message: &str, span: &Span) -> String {
    let mut error = String::new();
    error.push_str(&self.error_header(&span));
    error.push_str(message);
    error.push_str("\n\n");
    error.push_str(&highlight_error(&self.file_content, span.start, span.end));
    error.push('\n');
    error
  }

  fn error_header(&self, span: &Span) -> String {
    format!(
      "\x1b[38;5;4m{}\x1b[0m:\x1b[38;5;5m{}\x1b[0m:\x1b[38;5;5m{}\x1b[0m\x1b[1;31m ERROR\x1b[0m ",
      self.file_path, span.line, span.column
    )
  }
}
