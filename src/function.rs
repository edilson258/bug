use crate::bytecode::Bytecode;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub code: Bytecode,
    pub max_stack: usize,
    pub max_locals: usize,
    pub arity: usize,
}

impl Function {
    pub fn make(
        name: String,
        arity: usize,
        max_stack: usize,
        max_locals: usize,
        code: Bytecode,
    ) -> Self {
        Self {
            name,
            arity,
            code,
            max_stack,
            max_locals,
        }
    }
}
