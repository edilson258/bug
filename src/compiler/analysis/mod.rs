mod context;
mod errorhandler;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::*;
use context::{Context, ContextType, Object};
use errorhandler::{AnalyserError, AnalyserErrors};
use spider_vm::stdlib::{FnPrototype, Type};

pub struct Analyser {
    context: Rc<RefCell<Context>>,
    typestack: Vec<Type>,
}

impl Analyser {
    pub fn make() -> Self {
        Self {
            context: Rc::new(RefCell::new(Context::make_global())),
            typestack: vec![],
        }
    }

    pub fn analyse(&mut self, ast: &mut AST) -> Result<(), AnalyserErrors> {
        let mut errors: AnalyserErrors = vec![];
        for stmt in ast {
            self.analyse_statement(stmt)
                .map_err(|err| errors.push(err))
                .ok();
        }

        if !self.has_main_fn() {
            errors.push(AnalyserError::name_error(format!(
                "Missing 'main' function"
            )));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn has_main_fn(&self) -> bool {
        match self.context.borrow().lookup("main") {
            Some(obj) => match obj {
                Object::FnPrototype(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn analyse_statement(&mut self, stmt: &mut Statment) -> Result<(), AnalyserError> {
        match stmt {
            Statment::FunctionDeclaration(fn_decl) => self.analyse_function_declaration(fn_decl),
            Statment::Expression(expression) => self.analyse_expression(expression),
        }
    }

    fn analyse_function_declaration(
        &mut self,
        fn_decl: &mut FunctionDeclaration,
    ) -> Result<(), AnalyserError> {
        if self.context.borrow().type_ != ContextType::Global {
            return Err(AnalyserError::illegal_decl(format!(
                "Functions must be only declared in global scope"
            )));
        }

        if self.context.borrow().is_declared(&fn_decl.name) {
            return Err(AnalyserError::name_error(format!(
                "'{}' is already bound",
                &fn_decl.name
            )));
        }

        self.context.borrow_mut().declare(
            fn_decl.name.clone(),
            Object::FnPrototype(FnPrototype {
                arity: fn_decl.params.len() as u8,
                argtypes: fn_decl
                    .params
                    .iter()
                    .map(|param| param.type_.clone())
                    .collect(),
                return_type: fn_decl.return_type.clone(),
            }),
        );

        let global_context = Rc::clone(&self.context);
        let fn_context = Rc::new(RefCell::new(Context::make(
            ContextType::Function,
            Rc::clone(&global_context),
        )));
        self.context = fn_context;

        for param in &fn_decl.params {
            self.context
                .borrow_mut()
                .declare(param.name.clone(), Object::VarType(param.type_.clone()))
        }

        for stmt in &mut fn_decl.body {
            if let Err(err) = self.analyse_statement(stmt) {
                self.typestack.clear();
                self.context = global_context;
                return Err(err);
            }
        }

        if self.typestack.is_empty() {
            if fn_decl.return_type != Type::Void {
                self.typestack.clear();
                self.context = global_context;
                return Err(AnalyserError::type_error(format!(
                    "Missing return val for a non-void function '{}'",
                    &fn_decl.name
                )));
            }
        } else {
            if self.typestack.len() > 1 {
                self.typestack.clear();
                self.context = global_context;
                return Err(AnalyserError::unhandled_stack(format!(
                    "Unhandled stack values for function '{}'",
                    &fn_decl.name
                )));
            }

            let provided_type = self.typestack.pop().unwrap();

            if provided_type != fn_decl.return_type {
                self.typestack.clear();
                self.context = global_context;
                return Err(AnalyserError::type_error(format!(
                    "Function '{}' expects return type {} but provided {}",
                    &fn_decl.name, fn_decl.return_type, provided_type
                )));
            }
        }

        self.context = global_context;
        Ok(())
    }

    fn analyse_expression(&mut self, expression: &mut Expression) -> Result<(), AnalyserError> {
        if self.context.borrow().type_ != ContextType::Function {
            return Err(AnalyserError::out_fn_expr(format!(
                "Expressions must be inside of functions"
            )));
        }

        match expression {
            Expression::Literal(literal) => self.analyse_literal_expression(literal),
            Expression::FunctionCall(fn_name) => self.analyse_function_call(fn_name),
            Expression::BinaryOp(binop) => self.analyse_binop(binop),
            Expression::Identifier(ident) => {
                let variable = self.context.borrow().lookup(&ident);
                if variable.is_none() {
                    return Err(AnalyserError::name_error(format!("'{}' is unbound", ident)));
                }
                match variable.unwrap() {
                    Object::FnPrototype(_) => todo!(),
                    Object::VarType(type_) => self.typestack.push(type_),
                }
                Ok(())
            }
        }
    }

    fn analyse_binop(&mut self, binop: &mut BinaryOp) -> Result<(), AnalyserError> {
        if self.typestack.len() > 2 {
            return Err(AnalyserError::unhandled_stack(format!(
                "Must handle stack values before '{:#?}' operation",
                binop
            )));
        }

        if self.typestack.len() < 2 {
            return Err(AnalyserError::unhandled_stack(format!(
                "Missing operands for '{:#?}' operation",
                binop
            )));
        }

        let rhs = self.typestack.pop().unwrap();
        let lhs = self.typestack.pop().unwrap();

        if rhs != lhs {
            return Err(AnalyserError::type_error(format!(
                "Operands of '{:#?}' operation must be of same type, but provided '{}' and '{}'",
                binop, lhs, rhs
            )));
        }

        match binop {
            BinaryOp::Plus(type_) => match lhs {
                Type::Integer => {
                    *type_ = Some(Type::Integer);
                    self.typestack.push(lhs)
                }
                _ => {
                    return Err(AnalyserError::type_error(format!(
                        "'{:#?}' operation not supported for '{}' type",
                        binop, lhs
                    )))
                }
            },
        }

        Ok(())
    }

    fn analyse_literal_expression(&mut self, literal: &mut Literal) -> Result<(), AnalyserError> {
        match literal {
            Literal::Int(_) => self.typestack.push(Type::Integer),
            Literal::String(_) => self.typestack.push(Type::String),
        }
        Ok(())
    }

    fn analyse_function_call(&mut self, fn_name: &mut String) -> Result<(), AnalyserError> {
        let func = self.context.borrow().lookup(&fn_name);
        if func.is_none() {
            return Err(AnalyserError::name_error(format!(
                "'{}' is unbound",
                fn_name
            )));
        }
        let prototype = match func.unwrap() {
            Object::FnPrototype(prototype) => prototype,
            _ => {
                return Err(AnalyserError::type_error(format!(
                    "'{}' is not callable",
                    fn_name
                )))
            }
        };

        if (self.typestack.len() as u8) < prototype.arity {
            return Err(AnalyserError::arg_error(format!(
                "Missing arguments for function '{}'",
                fn_name
            )));
        }

        if (self.typestack.len() as u8) > prototype.arity {
            return Err(AnalyserError::unhandled_stack(format!(
                "Unhandled stack values calling '{}'",
                fn_name
            )));
        }

        self.typestack.clear();
        if prototype.return_type != Type::Void {
            self.typestack.push(prototype.return_type.clone());
        }

        Ok(())
    }
}
