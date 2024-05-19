use std::process::exit;

use crate::bytecode::{Bytecode, Instr};
use crate::object::{Object, DEFAULT_OBJECT};
use crate::stack::{OpStack, MAX_STACK};

/// The max number of local variables a function can hold
///
const MAX_LOCALS: usize = 25;

#[derive(Debug, Clone)]
pub struct Locals {
    count: usize,
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
            count: 0,
            max_locals: cap,
            inner: [DEFAULT_OBJECT; MAX_LOCALS],
        }
    }

    pub fn append(&mut self, local: Object) {
        if self.max_locals >= self.inner.len() {
            eprintln!("[Error]: Couldn't append to locals: NoEnoughSpace");
            exit(1);
        }
        self.inner[self.count] = local;
        self.count += 1;
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

    /// Note: this function does check bounds
    ///
    pub fn store_at(&mut self, index: usize, o: Object) {
        self.inner[index] = o;
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub pc: usize,
    code: Bytecode,
    pub locals: Locals,
    pub opstack: OpStack,
}

impl Frame {
    pub fn make(code: Bytecode, max_locals: usize, max_stack: usize) -> Self {
        Self {
            pc: 0,
            code,
            opstack: OpStack::make(max_stack),
            locals: Locals::make(max_locals),
        }
    }

    pub fn fetch_next_instr(&mut self) -> Instr {
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

    pub fn locals_append(&mut self, o: Object) {
        self.locals.append(o);
    }
}

pub const DEFAULT_FRAME: Frame = Frame {
    pc: 0,
    code: Bytecode { instrs: vec![] },
    locals: Locals {
        count: 0,
        max_locals: 0,
        inner: [DEFAULT_OBJECT; MAX_LOCALS],
    },
    opstack: OpStack {
        sp: 0,
        max_stack: 0,
        inner: [DEFAULT_OBJECT; MAX_STACK],
    },
};
