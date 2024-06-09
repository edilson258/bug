mod metascope;

use crate::ast::{
    BlockStatment, Expression, FunctionCall, FunctionDeclaration as FnDecl, Infix, Literal,
    Statment, AST,
};
use core::fmt;
use metascope::{MetaFunction, MetaObject, MetaScope};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Integer,
    String,
    Void,
}

#[derive(Debug)]
enum AnaliserErrorKind {
    Type,
    Name,
    Arguments,
}

impl fmt::Display for AnaliserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "[Type Error]"),
            Self::Name => write!(f, "[Name Error]"),
            Self::Arguments => write!(f, "[Argument Error]"),
        }
    }
}

pub struct AnaliserError {
    kind: AnaliserErrorKind,
    msg: String,
}

impl AnaliserError {
    pub fn type_error(msg: String) -> Self {
        Self {
            kind: AnaliserErrorKind::Type,
            msg,
        }
    }

    pub fn name_error(msg: String) -> Self {
        Self {
            kind: AnaliserErrorKind::Name,
            msg,
        }
    }

    pub fn args_error(msg: String) -> Self {
        Self {
            kind: AnaliserErrorKind::Arguments,
            msg,
        }
    }
}

impl fmt::Display for AnaliserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub type AnaliserErrors = Vec<AnaliserError>;

pub struct Analiser {
    scope: MetaScope,
}

impl Analiser {
    pub fn make() -> Self {
        Self {
            scope: MetaScope::new(),
        }
    }

    pub fn analise(&mut self, ast: &AST) -> Result<(), AnaliserErrors> {
        let mut errors: AnaliserErrors = vec![];

        for stmt in ast {
            self.analise_statment(stmt)
                .map_err(|err| errors.push(err))
                .ok();
        }

        if errors.is_empty() {
            return Ok(());
        }

        Err(errors)
    }

    fn analise_statment(&mut self, stmt: &Statment) -> Result<Type, AnaliserError> {
        match stmt {
            Statment::FunctionDeclaration(fn_decl) => self.analise_function_declaration(fn_decl),
            Statment::Expression(expr) => self.analise_expression(expr),
        }
    }

    fn analise_function_declaration(&mut self, fn_decl: &FnDecl) -> Result<Type, AnaliserError> {
        if self.scope.exists_in_current(&fn_decl.name) {
            return Err(AnaliserError::name_error(format!(
                "'{}' is already bound",
                fn_decl.name
            )));
        }

        let expected_return_type = fn_decl.return_type.clone();
        self.analise_block_statment(&fn_decl.body, &expected_return_type)?;

        self.scope.insert(
            fn_decl.name.clone(),
            MetaObject::MetaFunction(MetaFunction {
                arity: 0,
                return_type: expected_return_type.clone(),
            }),
        );

        Ok(expected_return_type)
    }

    fn analise_block_statment(
        &mut self,
        block: &BlockStatment,
        expected_type: &Type,
    ) -> Result<(), AnaliserError> {
        if block.is_empty() && *expected_type != Type::Void {
            return Err(AnaliserError::type_error(format!(
                "Missing return from non-void block"
            )));
        }

        /* @Note:
         * The return type is equal to the last block Statment
         * because we don't support flow control mechanismis like  ifs and return.
         */
        let mut last_statment_type = Type::Void;

        for statment in block {
            last_statment_type = self.analise_statment(statment)?;
        }

        if last_statment_type != *expected_type {
            return Err(AnaliserError::type_error(format!("Return type miss match")));
        }

        Ok(())
    }

    fn analise_expression(&mut self, expression: &Expression) -> Result<Type, AnaliserError> {
        match expression {
            Expression::Literal(literal) => Ok(self.analise_literal_expression(literal)),
            Expression::Infix(lhs, infix, rhs) => self.analise_infix_expression(lhs, infix, rhs),
            Expression::FunctionCall(fn_call) => self.analyse_function_call(fn_call),
            _ => todo!(),
        }
    }

    fn analyse_function_call(&mut self, fn_call: &FunctionCall) -> Result<Type, AnaliserError> {
        let object = match self.scope.lookup_global(&fn_call.name) {
            Some(obj) => obj,
            None => {
                return Err(AnaliserError::name_error(format!(
                    "'{}' is unbound",
                    fn_call.name
                )))
            }
        };

        let function_object = match object {
            MetaObject::MetaFunction(fn_obj) => fn_obj,
        };

        if function_object.arity != fn_call.args.len() as u8 {
            return Err(AnaliserError::args_error(format!(
                "'{}' expects {} args but provided {}",
                fn_call.name,
                function_object.arity,
                fn_call.args.len()
            )));
        }

        Ok(function_object.return_type.clone())
    }

    fn analise_literal_expression(&mut self, literal: &Literal) -> Type {
        match literal {
            Literal::Int(_) => Type::Integer,
            Literal::String(_) => Type::String,
        }
    }

    fn analise_infix_expression(
        &mut self,
        lhs: &Expression,
        infix: &Infix,
        rhs: &Expression,
    ) -> Result<Type, AnaliserError> {
        let lhs_type = self.analise_expression(lhs)?;
        let rhs_type = self.analise_expression(rhs)?;

        if lhs_type != rhs_type {
            return Err(AnaliserError::type_error(format!(
                "'{}' operands must be of same type",
                infix
            )));
        }

        Ok(lhs_type)
    }
}
