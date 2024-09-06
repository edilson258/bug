use super::super::ast::*;
use crate::frontend::token::Location;
use bug::stdlib::{list_native_fns, NativeFn};
use bug::{FnPrototype, Type};
use std::collections::HashMap;

pub struct Checker<'a> {
  ast: &'a Ast,
  ctx: Context<'a>,
  stack_depth: usize,
  diagnostics: Diagnostics<'a>,
}

impl<'a> Checker<'a> {
  pub fn new(ast: &'a Ast) -> Self {
    Self { ast, ctx: Context::new(list_native_fns()), stack_depth: 0, diagnostics: Diagnostics::new() }
  }

  pub fn check(&mut self) -> &Diagnostics {
    for statement in self.ast {
      self.check_statement(statement);
    }
    self.lookup_main_function();
    &self.diagnostics
  }

  fn lookup_main_function(&mut self) {
    let main = self.ctx.lookup("main");

    if main.is_none() {
      self.emit_missing_main();
      return;
    }

    let (arity, return_type) = match &main.unwrap().value {
      SymbolValue::Function(function) => (function.arity, function.return_type.clone()),
    };

    if arity != 0 {
      self.emit_main_takes_no_args();
    }

    if return_type != Type::Void {
      self.emit_main_have_no_return();
    }
  }

  fn check_statement(&mut self, statement: &'a Statement) {
    match statement {
      Statement::Function(function) => self.check_statement_function(function),
      Statement::Expression(expression) => self.check_statement_expression(expression),
    }
  }

  fn check_statement_function(&mut self, function: &'a StatementFunction) {
    if let Some(symb) = self.ctx.lookup(function.get_name()) {
      if symb.is_native || symb.scope_id == self.ctx.curr_scope_id {
        self.emit_name_bound(&function.identifier.location, function.get_name());
      }
    }

    let fn_proto = FnPrototype { arity: 0, argtypes: vec![], return_type: Type::Void };
    let fn_symbol = Symbol::new(self.ctx.curr_scope_id, false, true, SymbolValue::Function(fn_proto));
    self.ctx.assign(function.get_name(), fn_symbol);

    self.ctx.enter_scope(ScopeKind::Function);

    for statement in &function.body {
      self.check_statement(statement);
    }

    self.ctx.leave_scope();
  }

  fn check_statement_expression(&mut self, expression: &'a StatementExpression) {
    match expression {
      StatementExpression::Call(call) => self.check_expression_call(call),
      StatementExpression::Literal(literal) => self.check_expression_literal(literal),
    }
  }

  fn check_expression_call(&mut self, call: &'a ExpressionCall) {
    let symb = self.ctx.lookup(call.get_name());

    if symb.is_none() {
      self.emit_name_unbound(&call.name_token.location, call.get_name());
      return;
    }

    let callee = match &symb.unwrap().value {
      SymbolValue::Function(x) => x,
    };

    if self.stack_depth < callee.arity as usize {
      let expected_count = callee.arity;
      self.emit_invalid_args_count(
        &call.name_token.location,
        call.get_name(),
        expected_count as usize,
        self.stack_depth,
      );
      return;
    }

    self.stack_depth -= callee.arity as usize;

    if callee.return_type != Type::Void {
      self.stack_depth += 1
    }
  }

  fn check_expression_literal(&mut self, _literal: &'a ExpressionLiteral) {
    self.stack_depth += 1;
  }
}

/**
 *
 * Diagnostics
 *
 */

impl<'a> Checker<'a> {
  fn emit_name_bound(&mut self, location: &'a Location, name: &str) {
    let message = format!("Redifinition of name `{name}`");
    self.diagnostics.emit(Diagnostic::Error(Error::new(Some(location), message)));
  }

  fn emit_name_unbound(&mut self, location: &'a Location, name: &str) {
    let message = format!("Cannot find name `{name}`");
    self.diagnostics.emit(Diagnostic::Error(Error::new(Some(location), message)));
  }

  fn emit_invalid_args_count(&mut self, location: &'a Location, name: &str, expected: usize, found: usize) {
    let message = format!("Function `{name}` expects `{expected}` args but got `{found}`");
    self.diagnostics.emit(Diagnostic::Error(Error::new(Some(location), message)));
  }

  fn emit_missing_main(&mut self) {
    let message = format!("Missing `main` function");
    self.diagnostics.emit(Diagnostic::Error(Error::new(None, message)));
  }

  fn emit_main_takes_no_args(&mut self) {
    let message = format!("The `main` function must not take any parameter");
    self.diagnostics.emit(Diagnostic::Error(Error::new(None, message)));
  }

  fn emit_main_have_no_return(&mut self) {
    let message = format!("The `main` function must return `void`");
    self.diagnostics.emit(Diagnostic::Error(Error::new(None, message)));
  }
}

#[derive(Debug)]
struct Error<'a> {
  message: String,
  location: Option<&'a Location>,
}

impl<'a> Error<'a> {
  fn new(location: Option<&'a Location>, message: String) -> Self {
    Self { location, message }
  }
}

#[derive(Debug)]
enum Diagnostic<'a> {
  Error(Error<'a>),
}

#[derive(Debug)]
pub struct Diagnostics<'a> {
  inner: Vec<Diagnostic<'a>>,
}

impl<'a> Diagnostics<'a> {
  fn new() -> Self {
    Self { inner: vec![] }
  }

  fn emit(&mut self, diagnostic: Diagnostic<'a>) {
    self.inner.push(diagnostic)
  }

  /**
   * Will display all the diagnostics to stdout and return the count of errors
   *
   */
  pub fn emit_all(&self) -> usize {
    self.inner.len()
  }
}

/**
 *
 * Context
 *
 */

enum ScopeKind {
  Gloabal,
  Function,
}

enum SymbolValue {
  Function(FnPrototype),
}

struct Symbol {
  scope_id: usize,
  is_const: bool,
  is_native: bool,
  value: SymbolValue,
}

impl Symbol {
  fn new(scope_id: usize, is_native: bool, is_const: bool, value: SymbolValue) -> Self {
    Self { scope_id, is_native, is_const, value }
  }
}

struct Scope<'a> {
  id: usize,
  kind: ScopeKind,
  unused_names: Vec<&'a str>,
}

impl<'a> Scope<'a> {
  fn new(id: usize, kind: ScopeKind) -> Self {
    Self { id, kind, unused_names: vec![] }
  }
}

struct Context<'a> {
  curr_scope_id: usize,
  table: HashMap<String, Symbol>,
  scopes: Vec<Scope<'a>>,
}

impl<'a> Context<'a> {
  fn new(natives: HashMap<String, NativeFn>) -> Self {
    let mut table: HashMap<String, Symbol> = HashMap::new();

    for (name, native) in natives {
      table.insert(name, Symbol::new(0, true, true, SymbolValue::Function(native.prototype)));
    }

    Self { curr_scope_id: 0, table, scopes: vec![Scope::new(0, ScopeKind::Gloabal)] }
  }

  fn lookup(&self, name: &str) -> Option<&Symbol> {
    self.table.get(name)
  }

  fn enter_scope(&mut self, kind: ScopeKind) {
    self.curr_scope_id += 1;
    self.scopes.push(Scope::new(self.curr_scope_id, kind));
  }

  fn leave_scope(&mut self) {
    self.curr_scope_id = self.curr_scope_id.checked_sub(1).unwrap_or(0);
    self.scopes.pop();
  }

  fn assign(&mut self, name: &'a str, symb: Symbol) {
    self.scopes[self.curr_scope_id].unused_names.push(name);
    self.table.insert(name.to_string(), symb);
  }
}
