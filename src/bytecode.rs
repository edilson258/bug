use core::fmt;
use std::process::exit;

#[derive(Debug, Clone)]
pub enum Instr {
    // arithmetic
    IAdd,

    // control flow
    Return,
    IReturn,
    Invoke(String),

    // data handlers
    ILdc(usize),
    ILoad(usize),
    IStore(usize),
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instrs: Vec<Instr>,
}

impl Bytecode {
    pub fn make(instrs: Vec<Instr>) -> Self {
        Self { instrs }
    }

    pub fn fetch_by_index(&self, index: usize) -> Instr {
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

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IAdd => write!(f, "[iadd]"),
            Self::Return => write!(f, "[return]"),
            Self::IReturn => write!(f, "[ireturn]"),
            Self::Invoke(name) => write!(f, "[invoke] {}", name),
            Self::ILdc(index) => write!(f, "[ildc] {}", index),
            Self::ILoad(index) => write!(f, "[iload] {}", index),
            Self::IStore(index) => write!(f, "[istore] {}", index),
        }
    }
}
