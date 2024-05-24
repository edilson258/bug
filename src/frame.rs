use std::process::exit;

use crate::bytecode::{Bytecode, Opcode};
use crate::object::Object;
use crate::stack::Stack;

/// The max number of local variables a function can hold
///
const MAX_LOCALS: usize = 5;

#[derive(Debug, Clone)]
pub struct Locals {
    max_locals: usize,
    inner: [Object; MAX_LOCALS],
}

impl Locals {
    pub fn make(cap: usize) -> Self {
        if cap > MAX_LOCALS {
            eprintln!("[Error]: Locals cap is too high max_locals: {}", MAX_LOCALS);
            exit(1);
        }
        Self {
            max_locals: cap,
            inner: Default::default(),
        }
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

    pub fn get_as_ref(&mut self, index: usize) -> &mut Object {
        if index >= self.inner.len() {
            eprintln!(
                "[Error]: Couldn't access to locals by index {}: OutOfRange",
                index
            );
            exit(1);
        }
        &mut self.inner[index]
    }

    pub fn store_at(&mut self, index: usize, o: Object) {
        if index >= self.max_locals {
            eprintln!(
                "[Error]: Couldn't store to locals at index {}: OutOfRange",
                index
            );
            exit(1);
        }
        self.inner[index] = o;
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub pc: usize,
    code: Bytecode,
    pub locals: Locals,
    pub opstack: Stack<Object>,
}

impl Frame {
    pub fn make(code: Bytecode, max_locals: usize, max_stack: usize) -> Self {
        Self {
            pc: 0,
            code,
            opstack: Stack::make(max_stack),
            locals: Locals::make(max_locals),
        }
    }

    pub fn fetch_next_instr(&mut self) -> Opcode {
        let instr = self.code.fetch_by_index(self.pc);
        self.pc += 1;
        instr
    }

    pub fn stack_push(&mut self, o: Object) {
        self.opstack.push(o);
    }

    pub fn stack_pop(&mut self) -> Object {
        self.opstack.pop()
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame {
            pc: 0,
            code: Bytecode::make(vec![]),
            locals: Locals::make(0),
            opstack: Stack::make(0),
        }
    }
}
