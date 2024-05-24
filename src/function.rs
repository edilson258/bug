use crate::bytecode::Bytecode;

#[derive(Debug, Clone)]
pub struct Function {
    pub fn_pool_ref: usize,
    pub arity: usize,
    pub code: Bytecode,
    pub max_stack: usize,
    pub max_locals: usize,
}

impl Function {
    pub fn make(
        fn_pool_ref: usize,
        arity: usize,
        max_stack: usize,
        max_locals: usize,
        code: Bytecode,
    ) -> Self {
        Self {
            fn_pool_ref,
            arity,
            code,
            max_stack,
            max_locals,
        }
    }
}
