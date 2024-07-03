use core::fmt;
use std::{process::exit, usize};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    // No Operation
    Nop,

    // arithmetic
    IAdd,
    IMul,
    IDiv,

    // control flow
    Return,
    IReturn,
    Invoke(String),

    ICmpGT,
    JumpIfFalse(usize),

    // data handlers
    Ldc(usize),
    ILoad(usize),
    IStore(usize),
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
            Self::ILoad(index) => write!(f, "[iload] {}", index),
            Self::IStore(index) => write!(f, "[istore] {}", index),
            Self::Bipush(iconst) => write!(f, "[bipush] {}", iconst),
            Self::IMul => write!(f, "[imul]"),
            Self::IDiv => write!(f, "[idiv]"),
            Self::Ldc(usize) => write!(f, "[ldc] {}", usize),
            Self::ICmpGT => write!(f, "[icmpgt]"),
            Self::JumpIfFalse(usize) => write!(f, "jumpiffalse {usize}"),
        }
    }
}
