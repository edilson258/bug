use crate::bytecode::{Bytecode, Instr};

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

    pub fn append_instr(&mut self, instr: Instr) {
        self.code.append(instr);
    }

    pub fn append_many(&mut self, instrs: Vec<Instr>) {
        for instr in instrs {
            self.append_instr(instr);
        }
    }
}
