use crate::span::Span;
use bug::Type;

pub type Ast = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Function(StatementFunction),
    Expression(StatementExpression),
}

#[derive(Debug)]
pub struct StatementFunction {
    pub identifier: Identifier,
    pub return_type: Type,
    pub body: StatementBlock,
    pub parameters: Parameters,
    pub _signature_span: Span,
}

impl StatementFunction {
    pub fn new(
        identifier: Identifier,
        parameters: Parameters,
        return_type: Type,
        body: StatementBlock,
        signature_span: Span,
    ) -> Self {
        Self { identifier, parameters, return_type, body, _signature_span: signature_span }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub span: Span,
    pub label: String,
}

impl Identifier {
    pub fn new(span: Span, label: String) -> Self {
        Self { span, label }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub identifier: Identifier,
    pub typ: Type,
    pub _span: Span,
}

impl Parameter {
    pub fn new(identifier: Identifier, typ: Type, span: Span) -> Self {
        Self { identifier, typ, _span: span }
    }
}

#[derive(Debug)]
pub struct StatementBlock {
    pub span: Span,
    pub statements: Vec<Statement>,
}

impl StatementBlock {
    pub fn new() -> StatementBlock {
        Self { statements: vec![], span: Span::default() }
    }
}

#[derive(Debug)]
pub struct Parameters {
    pub parameters: Vec<Parameter>,
    pub span: Span,
}

impl Parameters {
    pub fn new() -> Self {
        Self { parameters: vec![], span: Span::default() }
    }
}

#[derive(Debug)]
pub enum StatementExpression {
    Call(ExpressionCall),
    Binary(ExpressionBinary),
    Literal(ExpressionLiteral),
    Identifier(Identifier),
    Ternary(ExpressionTernary),
}

impl StatementExpression {
    pub fn get_span(&self) -> Span {
        match &self {
            Self::Call(call) => call.span.clone(),
            Self::Binary(binary) => binary.span.clone(),
            Self::Literal(literal) => match literal {
                ExpressionLiteral::String(string) => string.span.clone(),
                ExpressionLiteral::Integer(integer) => integer.span.clone(),
            },
            Self::Identifier(identifier) => identifier.span.clone(),
            Self::Ternary(ternary) => ternary.span.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionTernary {
    pub consequence: Box<StatementExpression>,
    pub alternative: Box<StatementExpression>,
    pub span: Span,
}

impl ExpressionTernary {
    pub fn new(consequence: StatementExpression, alternative: StatementExpression, span: Span) -> Self {
        Self { consequence: Box::new(consequence), alternative: Box::new(alternative), span }
    }
}

#[derive(Debug)]
pub struct ExpressionCall {
    pub identifier: Identifier,
    pub span: Span,
}

impl ExpressionCall {
    pub fn new(span: Span, identifier: Identifier) -> Self {
        Self { span, identifier }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    GratherThan,
}

#[derive(Debug)]
pub struct ExpressionBinary {
    pub operator: BinaryOperator,
    pub span: Span,
    pub operands_types: Option<Type>,
}

impl ExpressionBinary {
    pub fn new(operator: BinaryOperator, span: Span) -> Self {
        Self { operator, span, operands_types: None }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::GratherThan => write!(f, ">"),
        }
    }
}

#[derive(Debug)]
pub enum ExpressionLiteral {
    String(LiteralString),
    Integer(LiteralInteger),
}

#[derive(Debug)]
pub struct LiteralString {
    pub span: Span,
    pub inner: String,
}

impl LiteralString {
    pub fn new(span: Span, inner: String) -> Self {
        Self { inner, span }
    }
}

#[derive(Debug)]
pub struct LiteralInteger {
    pub span: Span,
    pub inner: i32,
}

impl LiteralInteger {
    pub fn new(span: Span, inner: i32) -> Self {
        Self { span, inner }
    }
}
