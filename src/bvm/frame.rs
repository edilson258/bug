use std::process::exit;

use crate::stack::Stack;
use bug::bytecode::{Bytecode, Opcode};
use bug::Object;

#[derive(Debug, Clone)]
pub struct Locals {
    inner: Vec<Object>,
}

impl Locals {
    pub fn make(max_locals: usize) -> Self {
        let mut inner: Vec<Object> = Vec::with_capacity(max_locals);
        for _ in 0..max_locals {
            inner.push(Object::Int(0)) // init locals with zeros
        }
        Self { inner }
    }

    pub fn get_by_index(&self, index: usize) -> Object {
        if index >= self.inner.len() {
            eprintln!(
                "[Error]: Couldn't access to locals by index {}: OutOfRange",
                index
            );
            exit(1);
        }
        self.inner[index].clone()
    }

    pub fn store_at(&mut self, index: usize, o: Object) {
        self.inner[index] = o;
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub pc: usize,
    code: Bytecode,
    pub locals: Locals,
    pub stack: Stack<Object>,
}

impl Frame {
    pub fn make(code: Bytecode, max_locals: usize) -> Self {
        Self {
            pc: 0,
            code,
            stack: Stack::make(),
            locals: Locals::make(max_locals),
        }
    }

    pub fn fetch_next_instr(&mut self) -> Opcode {
        let instr = self.code.fetch_by_index(self.pc);
        self.pc += 1;
        instr
    }
}
