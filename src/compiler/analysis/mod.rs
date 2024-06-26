mod metascope;

use crate::ast::*;
use core::fmt;
use metascope::{MetaFunction, MetaObject, MetaScope};
use spider_vm::stdlib::Type;

#[derive(Debug)]
enum AnalyserErrorKind {
    Type,
    Name,
    Argument,
}

impl fmt::Display for AnalyserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "[Type Error]"),
            Self::Name => write!(f, "[Name Error]"),
            Self::Argument => write!(f, "[Argument Error]"),
        }
    }
}

pub struct AnalyserError {
    kind: AnalyserErrorKind,
    msg: String,
}

impl AnalyserError {
    pub fn type_error(msg: String) -> Self {
        Self {
            kind: AnalyserErrorKind::Type,
            msg,
        }
    }

    pub fn name_error(msg: String) -> Self {
        Self {
            kind: AnalyserErrorKind::Name,
            msg,
        }
    }

    pub fn arg_error(msg: String) -> Self {
        Self {
            kind: AnalyserErrorKind::Argument,
            msg,
        }
    }
}

impl fmt::Display for AnalyserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub type AnalyserErrors = Vec<AnalyserError>;

pub struct Analyser {
    scope: MetaScope,
}

impl Analyser {
    pub fn make() -> Self {
        Self {
            scope: MetaScope::new(),
        }
    }

    pub fn analyse(&mut self, ast: &AST) -> Result<(), AnalyserErrors> {
        let mut errors: AnalyserErrors = vec![];
        for stmt in ast {
            self.analyse_statment(stmt)
                .map_err(|err| errors.push(err))
                .ok();
        }
        let main_fn = self.scope.lookup_global("main");
        if main_fn.is_none() {
            errors.push(AnalyserError::name_error(format!(
                "Missing 'main' function"
            )));
        } else {
            match main_fn.unwrap() {
                MetaObject::MetaFunction(_) => {}
            };
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }

    fn analyse_statment(&mut self, stmt: &Statment) -> Result<Type, AnalyserError> {
        match stmt {
            Statment::FunctionDeclaration(fn_decl) => self.analyse_function_declaration(fn_decl),
            Statment::Expression(expr) => self.analyse_expression(expr),
        }
    }

    fn analyse_function_declaration(
        &mut self,
        fn_decl: &FunctionDeclaration,
    ) -> Result<Type, AnalyserError> {
        if self.scope.exists_in_current(&fn_decl.name) {
            return Err(AnalyserError::name_error(format!(
                "'{}' is already bound",
                fn_decl.name
            )));
        }

        let expected_return_type = fn_decl.return_type.clone();
        self.analyse_block_statment(&fn_decl.body, &expected_return_type)?;

        self.scope.insert(
            fn_decl.name.clone(),
            MetaObject::MetaFunction(MetaFunction {
                arity: 0,
                return_type: expected_return_type.clone(),
            }),
        );
        Ok(Type::Void)
    }

    fn analyse_block_statment(
        &mut self,
        block: &BlockStatment,
        expected_type: &Type,
    ) -> Result<Type, AnalyserError> {
        if block.is_empty() && *expected_type != Type::Void {
            return Err(AnalyserError::type_error(format!(
                "Missing return from non-void block"
            )));
        }

        let mut last_statment_type = Type::Void;

        for statment in block {
            last_statment_type = self.analyse_statment(statment)?;
        }

        if last_statment_type != *expected_type {
            return Err(AnalyserError::type_error(format!("Return type miss match")));
        }

        Ok(last_statment_type)
    }

    fn analyse_expression(&mut self, expression: &Expression) -> Result<Type, AnalyserError> {
        match expression {
            Expression::Literal(literal) => Ok(self.analyse_literal_expression(literal)),
            Expression::FunctionCall(fn_call) => self.analyse_function_call(fn_call),
            Expression::BinaryOp(op) => self.analyse_binop(op),
            _ => todo!(),
        }
    }

    fn analyse_binop(&mut self, _binop: &BinaryOp) -> Result<Type, AnalyserError> {
        if self.scope.typestack.len() < 2 {
            return Err(AnalyserError::arg_error(format!(
                "Missing operands for `+` op"
            )));
        }

        let rhs_type = self.scope.typestack.pop().unwrap();
        let lhs_type = self.scope.typestack.pop().unwrap();

        if lhs_type != rhs_type {
            return Err(AnalyserError::type_error(format!(
                "Operands of `+` must be of same type"
            )));
        }

        self.scope.typestack.push(rhs_type);

        Ok(lhs_type)
    }

    fn analyse_function_call(&mut self, fn_name: &String) -> Result<Type, AnalyserError> {
        let object = match self.scope.lookup_global(&fn_name) {
            Some(obj) => obj,
            None => {
                return Err(AnalyserError::name_error(format!(
                    "'{}' is unbound",
                    fn_name
                )))
            }
        };

        let function_object = match object {
            MetaObject::MetaFunction(fn_obj) => fn_obj,
        };

        if function_object.arity > self.scope.typestack.len() as u8 {
            return Err(AnalyserError::arg_error(format!(
                "Missing args for '{}' function",
                fn_name
            )));
        }

        let return_type = function_object.return_type.clone();

        for _ in 0..function_object.arity {
            self.scope.typestack.pop();
        }

        self.scope.typestack.push(return_type.clone());
        Ok(return_type)
    }

    fn analyse_literal_expression(&mut self, literal: &Literal) -> Type {
        match literal {
            Literal::Int(_) => {
                self.scope.typestack.push(Type::Integer);
                Type::Integer
            }
            Literal::String(_) => {
                self.scope.typestack.push(Type::String);
                Type::String
            }
        }
    }
}
