use crate::ast::*;
use bug::{stdlib::NativeFn, FunctionPrototype, Type};
use std::collections::HashMap;

pub struct Checker<'a> {
  ast: &'a Ast,
  ctx: Context,
}

impl<'a> Checker<'a> {
  pub fn new(ast: &'a Ast, natives: HashMap<String, NativeFn>) -> Self {
    Self { ast, ctx: Context::new(natives) }
  }

  pub fn check(&mut self) {
    for statement in self.ast {
      self.check_statement(statement);
    }
  }

  fn check_statement(&mut self, statement: &Statement) {
    match statement {
      Statement::Function(f) => self.check_statement_function(f),
      Statement::Expression(expression) => self.check_statement_expression(expression),
    };
  }

  fn check_statement_function(&mut self, f: &StatementFunction) {
    if let Some(_) = self.ctx.lookup_locally(&f.identifier.label) {
      eprintln!("[ERROR]: Name `{}` is already used", f.identifier.label);
      std::process::exit(1);
    }
    let param_types: Vec<Type> = f.parameters.parameters.iter().map(|p| p.typ.clone()).collect();
    let prototype = FunctionPrototype::new(param_types.len(), f.return_type.clone(), param_types);
    self.ctx.declare_locally(f.identifier.label.clone(), Symbol::Function(prototype));
    self.ctx.enter_scope(ScopeType::Function);
    for statement in &f.body.statements {
      self.check_statement(statement);
    }
    let returned_type = self.ctx.top_of_stack().unwrap_or(&Type::Void);
    if *returned_type != f.return_type {
      eprintln!("[ERROR]: Function `{}` expects `{}` but got `{}`", f.identifier.label, f.return_type, returned_type);
      std::process::exit(1);
    }
    self.ctx.leave_scope();
  }

  fn check_statement_expression(&mut self, expression: &StatementExpression) {
    match expression {
      StatementExpression::Call(call) => self.check_expression_call(call),
      StatementExpression::Binary(binary) => self.check_expression_binary(binary),
      StatementExpression::Literal(literal) => self.check_expression_literal(literal),
    };
  }

  fn check_expression_call(&mut self, call: &ExpressionCall) {
    let callee = self.ctx.lookup(&call.identifier.label);
    if callee.is_none() {
      eprintln!("[ERROR]: Name `{}` is not declared", call.identifier.label);
      std::process::exit(1);
    }
    let callee = match callee.unwrap() {
      Symbol::Function(f) => f.clone(),
    };
    if callee.arity != self.ctx.stack_depth() {
      eprintln!("[ERROR]: Missing arguments for function `{}`", call.identifier.label);
      std::process::exit(1);
    }
    let provided_args = self.ctx.pop_many(callee.arity);
    for (_, (provided, expected)) in provided_args.into_iter().zip(callee.parameters_types).enumerate() {
      if provided != expected {
        eprintln!("[ERROR]: Expecting argument of type `{}` but got `{}`", expected, provided);
      }
    }
    if callee.return_type != Type::Void {
      self.ctx.push(callee.return_type);
    }
  }

  fn check_expression_binary(&mut self, binary: &ExpressionBinary) {
    if self.ctx.stack_depth() < 2 {
      eprintln!("[ERROR]: Missing operands for `{}` operator", binary.operator);
      std::process::exit(1);
    }
    let rhs = self.ctx.pop();
    let lhs = self.ctx.pop();
    if lhs != rhs {
      eprintln!("[ERROR]: Operator `{}` doesn't apply to types `{}` and `{}`", binary.operator, lhs, rhs);
      std::process::exit(1);
    }
    match binary.operator {
      BinaryOperator::Plus => self.check_binary_plus(lhs, rhs),
      BinaryOperator::Minus => self.check_binary_minus(lhs, rhs),
    }
  }

  fn check_binary_plus(&mut self, lhs: Type, _rhs: Type) {
    match lhs {
      Type::Integer => self.ctx.push(Type::Integer),
      Type::String => self.ctx.push(Type::String),
      _ => {
        eprintln!("[ERROR]: Operator `+` doesn't apply to type `{}`", lhs);
        std::process::exit(1);
      }
    };
  }

  fn check_binary_minus(&mut self, lhs: Type, _rhs: Type) {
    match lhs {
      Type::Integer => self.ctx.push(Type::Integer),
      _ => {
        eprintln!("[ERROR]: Operator `-` doesn't apply to type `{}`", lhs);
        std::process::exit(1);
      }
    };
  }

  fn check_expression_literal(&mut self, literal: &ExpressionLiteral) {
    match literal {
      ExpressionLiteral::String(_) => self.ctx.push(Type::String),
      ExpressionLiteral::Integer(_) => self.ctx.push(Type::Integer),
    };
  }
}

enum Symbol {
  Function(FunctionPrototype),
}

enum ScopeType {
  Global,
  Function,
}

struct Scope {
  typ: ScopeType,
  table: HashMap<String, Symbol>,
  stack: Vec<Type>,
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

  fn declare_locally(&mut self, key: String, value: Symbol) {
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

  fn top_of_stack(&self) -> Option<&Type> {
    self.scopes[self.scope_pointer].stack.last()
  }

  fn stack_depth(&self) -> usize {
    self.scopes[self.scope_pointer].stack.len()
  }

  fn pop(&mut self) -> Type {
    self.scopes[self.scope_pointer].stack.pop().unwrap()
  }

  fn push(&mut self, typ: Type) {
    self.scopes[self.scope_pointer].stack.push(typ);
  }

  fn pop_many(&mut self, count: usize) -> Vec<Type> {
    let mut types: Vec<Type> = vec![];
    for _ in 0..count {
      types.push(self.scopes[self.scope_pointer].stack.pop().unwrap())
    }
    types
  }
}
