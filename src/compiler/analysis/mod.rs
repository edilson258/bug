use crate::ast::{Expression, Infix, Literal, Statment, AST};
use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Type {
    Integer,
    String,
}

#[derive(Debug)]
enum AnaliserErrorKind {
    Type,
}

impl fmt::Display for AnaliserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "[Type Error]"),
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
}

impl fmt::Display for AnaliserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

pub type AnaliserErrors = Vec<AnaliserError>;

pub struct Analiser {}

impl Analiser {
    pub fn make() -> Self {
        Self {}
    }

    pub fn analise(&mut self, ast: &AST) -> Result<(), AnaliserErrors> {
        let mut errors: AnaliserErrors = vec![];

        for stmt in ast {
            match self.analise_stmt(stmt) {
                Ok(()) => continue,
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            return Ok(());
        }

        Err(errors)
    }

    fn analise_stmt(&mut self, stmt: &Statment) -> Result<(), AnaliserError> {
        match stmt {
            Statment::Expression(expr) => match self.analise_expression(expr) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            },
            _ => todo!(),
        }
    }

    fn analise_expression(&mut self, expression: &Expression) -> Result<Type, AnaliserError> {
        match expression {
            Expression::Literal(literal) => Ok(self.analise_literal_expression(literal)),
            Expression::Infix(lhs, infix, rhs) => self.analise_infix_expression(lhs, infix, rhs),
            _ => todo!(),
        }
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
