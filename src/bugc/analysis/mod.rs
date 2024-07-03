mod context;
mod errorhandler;

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::*;
use bug::{FnPrototype, Type};
use context::{Context, ContextType, MetaObject};
use errorhandler::{AnalyserError, AnalyserErrors};

pub struct Analyser {
    context: Rc<RefCell<Context>>,
    metastack: Vec<Type>,
    expected_type: Type,
}

impl Analyser {
    pub fn make() -> Self {
        Self {
            context: Rc::new(RefCell::new(Context::make_global())),
            metastack: vec![],
            expected_type: Type::Void,
        }
    }

    pub fn analyse(&mut self, ast: &mut AST) -> Result<(), AnalyserErrors> {
        let mut errors: AnalyserErrors = vec![];
        for stmt in ast {
            self.analyse_statement(stmt)
                .map_err(|err| errors.push(err))
                .ok();
        }

        if !self.is_main_fn_declared() {
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

    fn is_main_fn_declared(&self) -> bool {
        match self.context.borrow().lookup("main") {
            Some(obj) => match obj {
                MetaObject::FnPrototype(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn analyse_statement(&mut self, stmt: &mut Statement) -> Result<(), AnalyserError> {
        match stmt {
            Statement::If(block) => self.analyse_if_statement(block),
            Statement::Expression(expression) => self.analyse_expression(expression),
            Statement::FunctionDeclaration(fn_decl) => self.analyse_function_declaration(fn_decl),
        }
    }

    fn analyse_if_statement(&mut self, block: &mut BlockStatement) -> Result<(), AnalyserError> {
        if self.metastack.is_empty() {
            return Err(AnalyserError::arg_error(format!(
                "Missing operand for 'if'"
            )));
        }
        if Type::Boolean != self.metastack.pop().unwrap() {
            return Err(AnalyserError::type_error(format!(
                "'if' expects boolean value on stack"
            )));
        }

        for stmt in block {
            self.analyse_statement(stmt)?;
        }

        if self.metastack.is_empty() {
            if self.expected_type != Type::Void {
                return Err(AnalyserError::type_error(format!(
                    "If block returns 'void' where '{}' is expected",
                    self.expected_type
                )));
            }
        } else {
            let provided_type = self.metastack.last().unwrap();
            if self.expected_type != *provided_type {
                return Err(AnalyserError::type_error(format!(
                    "If block returns '{}' where '{}' is expected",
                    provided_type, self.expected_type
                )));
            }
        }

        Ok(())
    }

    fn analyse_function_declaration(
        &mut self,
        fn_decl: &mut FunctionDeclaration,
    ) -> Result<(), AnalyserError> {
        self.metastack.clear();

        // 1. check the scope where it's declared
        if self.context.borrow().type_ != ContextType::Global {
            return Err(AnalyserError::illegal_decl(format!(
                "Functions must be only declared in global scope"
            )));
        }

        // 2. check if the name is already taken
        if self.context.borrow().is_declared(&fn_decl.name) {
            return Err(AnalyserError::name_error(format!(
                "'{}' is already bound",
                &fn_decl.name
            )));
        }

        self.context.borrow_mut().declare(
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

        let global_context = Rc::clone(&self.context);
        let fn_context = Rc::new(RefCell::new(Context::make(
            ContextType::Function,
            Rc::clone(&global_context),
        )));
        self.context = fn_context;
        self.expected_type = fn_decl.return_type.clone();

        for param in &fn_decl.params {
            self.context
                .borrow_mut()
                .declare(param.name.clone(), MetaObject::VarType(param.type_.clone()))
        }

        for stmt in &mut fn_decl.body {
            if let Err(err) = self.analyse_statement(stmt) {
                self.context = global_context;
                return Err(err);
            }
        }

        if self.metastack.is_empty() {
            if fn_decl.return_type != Type::Void {
                self.context = global_context;
                return Err(AnalyserError::type_error(format!(
                    "Missing return val for a non-void function '{}'",
                    &fn_decl.name
                )));
            }
        } else {
            let provided_type = self.metastack.pop().unwrap();
            if provided_type != fn_decl.return_type {
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
            Expression::Identifier(ident) => self.analyse_identifier(ident),
            Expression::Return(type_) => self.analyse_return_expression(type_),
        }
    }

    fn analyse_identifier(&mut self, ident: &mut String) -> Result<(), AnalyserError> {
        let object = self.context.borrow().lookup(&ident);
        if object.is_none() {
            return Err(AnalyserError::name_error(format!("'{}' is unbound", ident)));
        }
        match object.unwrap() {
            MetaObject::FnPrototype(_) => todo!(),
            MetaObject::VarType(type_) => self.metastack.push(type_),
        }
        Ok(())
    }

    fn analyse_binop(&mut self, binop: &mut BinaryOp) -> Result<(), AnalyserError> {
        if self.metastack.len() < 2 {
            return Err(AnalyserError::type_error(format!(
                "Missing operands for '{:#?}' operation",
                binop
            )));
        }

        let rhs = self.metastack.pop().unwrap();
        let lhs = self.metastack.pop().unwrap();

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
                    self.metastack.push(lhs)
                }
                _ => {
                    return Err(AnalyserError::type_error(format!(
                        "'{:#?}' operation not supported for '{}' type",
                        binop, lhs
                    )))
                }
            },
            BinaryOp::GratherThan(type_) => match lhs {
                Type::Integer => {
                    *type_ = Some(Type::Integer);
                    self.metastack.push(Type::Boolean);
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
            Literal::Int(_) => self.metastack.push(Type::Integer),
            Literal::String(_) => self.metastack.push(Type::String),
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
            MetaObject::FnPrototype(prototype) => prototype,
            _ => {
                return Err(AnalyserError::type_error(format!(
                    "'{}' is not callable",
                    fn_name
                )))
            }
        };

        if (self.metastack.len() as u8) < prototype.arity {
            return Err(AnalyserError::arg_error(format!(
                "Missing arguments for function '{}'",
                fn_name
            )));
        }

        self.metastack.clear();
        if prototype.return_type != Type::Void {
            self.metastack.push(prototype.return_type.clone());
        }

        Ok(())
    }

    fn analyse_return_expression(&mut self, type_: &mut Option<Type>) -> Result<(), AnalyserError> {
        *type_ = Some(self.metastack.last().unwrap_or(&Type::Void).clone());
        Ok(())
    }
}
