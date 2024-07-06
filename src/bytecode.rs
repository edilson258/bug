use core::fmt;
use std::{process::exit, usize};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    /// Will do nothing for a cycle
    Nop,
    /// Add two ints on top of the stack and push the result
    IAdd,
    /// Multiply two ints on top of the stack and push the result
    IMul,
    /// Substract two ints on top of the stack and push the result
    IDiv,
    /// Return from a frame (block)
    Return,
    /// Returns an int from a frame (block)
    IReturn,
    /// Will make a function call by provided name
    Invoke(String),
    /// Will compare the two ints on top of stack and set the bflag register to true if the first
    /// is grather than the second
    ICmpGT,
    /// Will jump to the provided offset if the bflag register if false
    JumpIfFalse(usize),
    /// Will Load a value from constant pool at provided index to the stack
    Ldc(usize),
    /// Will load a value from locals at provided index to the stack
    LLoad(usize),
    /// Will move a value from top of the stack to the locals at provided index
    LStore(usize),
    /// Will push an imediate int to the stack
    Bipush(i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bytecode {
    pub instrs: Vec<Opcode>,
}

impl Bytecode {
    pub fn make(instrs: Vec<Opcode>) -> Self {
        Self { instrs }
    }

    pub fn fetch_by_index(&self, index: usize) -> Opcode {
        self.check_bound(index);
        self.instrs[index].clone()
    }

    fn check_bound(&self, index: usize) {
        if index >= self.instrs.len() {
            eprintln!(
                "[Error]: Couldn't fetch instruction at index {}: OutOfRange",
                index
            );
            exit(1);
        }
    }

    pub fn push(&mut self, op: Opcode) {
        self.instrs.push(op);
    }

    pub fn push_many(&mut self, ops: Vec<Opcode>) {
        self.instrs.extend(ops);
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nop => write!(f, "[Nop]"),
            Self::IAdd => write!(f, "[iadd]"),
            Self::Return => write!(f, "[return]"),
            Self::IReturn => write!(f, "[ireturn]"),
            Self::Invoke(name) => write!(f, "[invoke] {}", name),
            Self::LLoad(index) => write!(f, "[iload] {}", index),
            Self::LStore(index) => write!(f, "[istore] {}", index),
            Self::Bipush(iconst) => write!(f, "[bipush] {}", iconst),
            Self::IMul => write!(f, "[imul]"),
            Self::IDiv => write!(f, "[idiv]"),
            Self::Ldc(usize) => write!(f, "[ldc] {}", usize),
            Self::ICmpGT => write!(f, "[icmpgt]"),
            Self::JumpIfFalse(usize) => write!(f, "jumpiffalse {usize}"),
        }
    }
}
