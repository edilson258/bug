mod errorhandler;
mod scope;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::*;
use bug::{FnPrototype, Type};
use errorhandler::{AnalyserError, AnalyserErrors};
use scope::{MetaObject, Scope, ScopeType};

#[derive(Debug, Clone)]
enum MetaStackEntry {
    Type(Type),
    Identifier(String, Type),
    VariableDeclaration(String, Type),
}

pub struct Analyser {
    scope: Rc<RefCell<Scope>>,
    metastack: Vec<MetaStackEntry>,
    errors: AnalyserErrors,
}

impl Analyser {
    pub fn make() -> Self {
        Self {
            scope: Rc::new(RefCell::new(Scope::make_global())),
            metastack: vec![],
            errors: vec![],
        }
    }

    pub fn analyse(&mut self, ast: &mut AST) -> Result<(), AnalyserErrors> {
        for stmt in ast {
            self.analyse_statement(stmt);
        }
        self.check_main_function();
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_main_function(&mut self) {
        let main_fn = self.scope.borrow().lookup("main");
        if main_fn.is_none() {
            self.errors.push(AnalyserError::name_error(format!(
                "Missing 'main' function"
            )));
            return;
        }

        if let MetaObject::FnPrototype(fn_prototype) = main_fn.unwrap() {
            if fn_prototype.arity != 0 || fn_prototype.return_type != Type::Void {
                self.errors.push(AnalyserError::type_error(format!(
                    "'main' function cannot accept args or return some value"
                )));
            }
        } else {
            self.errors.push(AnalyserError::name_error(format!(
                "'main' must be declared as function"
            )));
        }
    }

    fn analyse_statement(&mut self, stmt: &mut Statement) {
        match stmt {
            Statement::If(block) => self.analyse_if_statement(block),
            Statement::Expression(expression) => self.analyse_expression(expression),
            Statement::FunctionDeclaration(fn_decl) => self.analyse_function_declaration(fn_decl),
            Statement::VariableDeclaration(var_decl) => self.analyse_variable_declaration(var_decl),
            Statement::Assignment(target) => self.analyse_assignment(target),
        }
    }

    fn analyse_assignment(&mut self, target: &mut Option<String>) {
        if self.metastack.len() < 2 {
            self.errors.push(AnalyserError::arg_error(format!(
                "Assignment (=) expects two operands on stack"
            )));
            return;
        }

        let rhs_type = match self.metastack.pop().unwrap() {
            MetaStackEntry::Type(type_) => type_,
            MetaStackEntry::Identifier(_, type_) => type_,
            _ => {
                self.errors.push(AnalyserError::type_error(format!(
                    "right side of assignment must be an expression"
                )));
                return;
            }
        };

        let (lhs_name, lhs_type) = match self.metastack.pop().unwrap() {
            MetaStackEntry::Identifier(name, type_) => (name, type_),
            MetaStackEntry::VariableDeclaration(name, type_) => (name, type_),
            _ => {
                self.errors.push(AnalyserError::type_error(format!(
                    "Cannot assign to a non-variable"
                )));
                return;
            }
        };

        if lhs_type != rhs_type {
            self.errors.push(AnalyserError::type_error(format!(
                "Cannot assign value of type '{}' to variable '{}' which has type '{}'",
                rhs_type, lhs_name, lhs_type
            )));
            return;
        }

        *target = Some(lhs_name);
    }

    fn analyse_variable_declaration(&mut self, var_decl: &mut VariableDeclaration) {
        if self.scope.borrow().is_declared(&var_decl.name) {
            self.errors.push(AnalyserError::name_error(format!(
                "'{}' is already bound",
                var_decl.name
            )));
            return;
        }
        self.scope.borrow_mut().declare(
            var_decl.name.clone(),
            MetaObject::VarType(var_decl.type_.clone()),
        );
        self.metastack.push(MetaStackEntry::VariableDeclaration(
            var_decl.name.clone(),
            var_decl.type_.clone(),
        ));
    }

    fn analyse_if_statement(&mut self, block: &mut BlockStatement) {
        if self.metastack.is_empty() {
            self.errors.push(AnalyserError::arg_error(format!(
                "'if' expects boolean value on top of the stack"
            )));
            return;
        }

        let provided_type = match self.metastack.pop().unwrap() {
            MetaStackEntry::Type(type_) => type_,
            MetaStackEntry::Identifier(_, type_) => type_,
            _ => {
                self.errors.push(AnalyserError::arg_error(format!(
                    "'if' expects boolean value on top of the stack"
                )));
                return;
            }
        };

        if provided_type != Type::Boolean {
            self.errors.push(AnalyserError::type_error(format!(
                "'if' expects boolean value on top of the stack"
            )));
            return;
        }

        for stmt in block {
            self.analyse_statement(stmt);
        }

        if self.metastack.is_empty() {
            if self.scope.borrow().expected_type != Type::Void {
                self.errors.push(AnalyserError::type_error(format!(
                    "'if' block returns 'void' where '{}' is expected",
                    self.scope.borrow().expected_type
                )));
            }
        } else {
            let provided_type = match self.metastack.pop().unwrap() {
                MetaStackEntry::Type(type_) => type_,
                MetaStackEntry::Identifier(_, type_) => type_,
                MetaStackEntry::VariableDeclaration(_, _) => Type::Void,
            };
            if self.scope.borrow().expected_type != provided_type {
                self.errors.push(AnalyserError::type_error(format!(
                    "'if' block returns '{}' where '{}' is expected",
                    provided_type,
                    self.scope.borrow().expected_type
                )));
            }
        }
    }

    fn analyse_function_declaration(&mut self, fn_decl: &mut FunctionDeclaration) {
        self.metastack.clear();

        if self.scope.borrow().type_ != ScopeType::Global {
            self.errors.push(AnalyserError::illegal_decl(format!(
                "Functions must be only declared in global scope"
            )));
        }

        if self.scope.borrow().is_declared(&fn_decl.name) {
            self.errors.push(AnalyserError::name_error(format!(
                "'{}' is already bound",
                &fn_decl.name
            )));
        }

        self.scope.borrow_mut().declare(
            fn_decl.name.clone(),
            MetaObject::FnPrototype(FnPrototype {
                arity: fn_decl.params.len() as u8,
                argtypes: fn_decl
                    .params
                    .iter()
                    .map(|param| param.type_.clone())
                    .collect(),
                return_type: fn_decl.return_type.clone(),
            }),
        );

        let global_context = Rc::clone(&self.scope);
        let fn_context = Rc::new(RefCell::new(Scope::make(
            ScopeType::Function,
            fn_decl.return_type.clone(),
            Rc::clone(&global_context),
        )));
        self.scope = fn_context;

        for param in &fn_decl.params {
            if let Some(_) = self.scope.borrow().lookup(&param.name) {
                self.errors.push(AnalyserError::name_error(format!(
                    "Duplicated parameter name '{}' for function '{}'",
                    param.name, fn_decl.name
                )));
            }

            self.scope
                .borrow_mut()
                .declare(param.name.clone(), MetaObject::VarType(param.type_.clone()))
        }

        for stmt in &mut fn_decl.body {
            self.analyse_statement(stmt);
        }

        if self.metastack.is_empty() {
            if fn_decl.return_type != Type::Void {
                self.scope = global_context;
                self.errors.push(AnalyserError::type_error(format!(
                    "Missing return value for a non-void function '{}'",
                    &fn_decl.name
                )));
                return;
            }
        } else {
            let provided_type = match self.metastack.pop().unwrap() {
                MetaStackEntry::Type(type_) => type_,
                MetaStackEntry::Identifier(_, type_) => type_,
                MetaStackEntry::VariableDeclaration(_, _) => Type::Void,
            };
            if provided_type != fn_decl.return_type {
                self.scope = global_context;
                self.errors.push(AnalyserError::type_error(format!(
                    "Function '{}' expects return type {} but provided {}",
                    &fn_decl.name, fn_decl.return_type, provided_type
                )));
                return;
            }
        }

        self.scope = global_context;
    }

    fn analyse_expression(&mut self, expression: &mut Expression) {
        match expression {
            Expression::Literal(literal) => self.analyse_literal_expression(literal),
            Expression::FunctionCall(fn_name) => self.analyse_function_call(fn_name),
            Expression::BinaryOp(binop) => self.analyse_binop(binop),
            Expression::Identifier(ident) => self.analyse_identifier(ident),
            Expression::Return(type_) => self.analyse_return_expression(type_),
        }
    }

    fn analyse_identifier(&mut self, ident: &mut String) {
        let object = self.scope.borrow().lookup(&ident);
        if object.is_none() {
            self.errors
                .push(AnalyserError::name_error(format!("'{}' is unbound", ident)));
            return;
        }
        match object.unwrap() {
            MetaObject::FnPrototype(_) => todo!(),
            MetaObject::VarType(type_) => self
                .metastack
                .push(MetaStackEntry::Identifier(ident.clone(), type_)),
        }
    }

    fn analyse_binop(&mut self, binop: &mut BinaryOp) {
        if self.metastack.len() < 2 {
            self.errors.push(AnalyserError::type_error(format!(
                "Missing operands for '{}' operation",
                binop
            )));
            return;
        }

        let rhs_type = match self.metastack.pop().unwrap() {
            MetaStackEntry::Type(type_) => type_,
            MetaStackEntry::Identifier(_, type_) => type_,
            _ => {
                self.errors.push(AnalyserError::type_error(format!(
                    "Right side of '{}' operation must an expression",
                    binop
                )));
                return;
            }
        };
        let lhs_type = match self.metastack.pop().unwrap() {
            MetaStackEntry::Type(type_) => type_,
            MetaStackEntry::Identifier(_, type_) => type_,
            _ => {
                self.errors.push(AnalyserError::type_error(format!(
                    "Left side of '{}' operation must an expression",
                    binop
                )));
                return;
            }
        };

        if lhs_type != rhs_type {
            self.errors.push(AnalyserError::type_error(format!(
                "Operands of '{}' operation must be of same type, but provided '{}' and '{}'",
                binop, lhs_type, rhs_type
            )));
        }

        match binop {
            BinaryOp::Plus(type_) => match lhs_type {
                Type::Integer => {
                    *type_ = Some(Type::Integer);
                    self.metastack.push(MetaStackEntry::Type(lhs_type));
                }
                _ => self.errors.push(AnalyserError::type_error(format!(
                    "'{}' operation not supported for '{}' type",
                    binop, lhs_type
                ))),
            },
            BinaryOp::GratherThan(type_) => match lhs_type {
                Type::Integer => {
                    *type_ = Some(Type::Integer);
                    self.metastack.push(MetaStackEntry::Type(Type::Boolean));
                }
                _ => self.errors.push(AnalyserError::type_error(format!(
                    "'{}' operation not supported for '{}' type",
                    binop, lhs_type
                ))),
            },
        }
    }

    fn analyse_literal_expression(&mut self, literal: &mut Literal) {
        match literal {
            Literal::Int(_) => self.metastack.push(MetaStackEntry::Type(Type::Integer)),
            Literal::String(_) => self.metastack.push(MetaStackEntry::Type(Type::String)),
            Literal::Boolean(_) => self.metastack.push(MetaStackEntry::Type(Type::Boolean)),
        }
    }

    fn analyse_function_call(&mut self, fn_name: &mut String) {
        let func = self.scope.borrow().lookup(&fn_name);
        if func.is_none() {
            self.errors.push(AnalyserError::name_error(format!(
                "'{}' is unbound",
                fn_name
            )));
        }
        let prototype = match func.unwrap() {
            MetaObject::FnPrototype(prototype) => prototype,
            _ => {
                self.errors.push(AnalyserError::type_error(format!(
                    "'{}' is not callable",
                    fn_name
                )));
                return;
            }
        };

        if (self.metastack.len() as u8) < prototype.arity {
            self.errors.push(AnalyserError::arg_error(format!(
                "Missing arguments for function '{}'",
                fn_name
            )));
            return;
        }

        for (index, (expected_type, provided_type)) in
            prototype.argtypes.iter().zip(&self.metastack).enumerate()
        {
            let provided_type = match provided_type {
                MetaStackEntry::Type(type_) => type_,
                MetaStackEntry::Identifier(_, type_) => type_,
                _ => {
                    self.errors.push(AnalyserError::arg_error(format!(
                        "Function argument must an expression",
                    )));
                    return;
                }
            };

            if expected_type != provided_type {
                self.errors.push(AnalyserError::type_error(format!(
                    "The {} parameter of '{}' function expected to be of type '{}' but provided value of type '{}'",
                    index + 1,
                    fn_name,
                    expected_type,
                    provided_type
                )));
                return;
            }
        }

        for _ in 0..prototype.arity {
            self.metastack.pop();
        }

        if prototype.return_type != Type::Void {
            self.metastack
                .push(MetaStackEntry::Type(prototype.return_type.clone()));
        }
    }

    fn analyse_return_expression(&mut self, type_: &mut Option<Type>) {
        match self.metastack.last() {
            Some(metatype) => {
                *type_ = Some(match metatype {
                    MetaStackEntry::Type(type_) => type_.clone(),
                    MetaStackEntry::Identifier(_, type_) => type_.clone(),
                    _ => unreachable!(),
                })
            }
            None => *type_ = Some(Type::Void),
        }
    }
}
