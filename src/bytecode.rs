use core::fmt;
use std::{process::exit, usize};

#[derive(Debug, Clone)]
pub enum Opcode {
    // arithmetic
    IAdd,
    IMul,
    IDiv,
    IIncr(usize, i32),

    // control flow
    Return,
    IReturn,
    Invoke(usize),

    // jumps
    Goto(usize),
    IfICmpLT(usize),
    IfICmpGT(usize),
    IfICmpE(usize),
    IfICmpNE(usize),

    // data handlers
    Ldc(usize),
    ILdc(usize),
    ILoad(usize),
    IStore(usize),
    Bipush(i32),
}

#[derive(Debug, Clone)]
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
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IAdd => write!(f, "[iadd]"),
            Self::Return => write!(f, "[return]"),
            Self::IReturn => write!(f, "[ireturn]"),
            Self::Invoke(name) => write!(f, "[invoke] {}", name),
            Self::ILdc(index) => write!(f, "[ildc] {}", index),
            Self::ILoad(index) => write!(f, "[iload] {}", index),
            Self::IStore(index) => write!(f, "[istore] {}", index),
            Self::Goto(index) => write!(f, "[jump] {}", index),
            Self::IfICmpLT(index) => write!(f, "[IfICmpLT] {}", index),
            Self::IfICmpGT(index) => write!(f, "[IfICmpGT] {}", index),
            Self::IfICmpE(index) => write!(f, "[IfICmpE] {}", index),
            Self::IfICmpNE(index) => write!(f, "[IfICmpNE] {}", index),
            Self::IIncr(index, iconst) => write!(f, "[incr] {} by {}", index, iconst),
            Self::Bipush(iconst) => write!(f, "[bipush] {}", iconst),
            Self::IMul => write!(f, "[imul]"),
            Self::IDiv => write!(f, "[idiv]"),
            Self::Ldc(usize) => write!(f, "[ldc] {}", usize),
        }
    }
}
