use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PushOperand {
    Integer(i32),
    Boolean(bool),
}

impl fmt::Display for PushOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(x) => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x),
        }
    }
}

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
    /// Returns the value on the top of the current stack
    ReturnTop,
    /// Will make a function call by provided name
    Invoke(String),
    /// Will compare the two ints on top of stack and set the bflag register to true if the first
    /// is grather than the second
    ICmpGT,
    // Will jump to the provided offset
    Jump(usize),
    /// Will jump to the provided offset if the top of stack is a bool value false
    JumpIfFalse(usize),
    /// Will Load a value from constant pool at provided index to the stack
    Ldc(usize),
    /// Will load a value from locals at provided index to the stack
    LLoad(usize),
    /// Will move a value from top of the stack to the locals at provided index
    LStore(usize),
    /// Will push an imediate value to the stack
    Push(PushOperand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteCodeStream {
    code: Vec<Opcode>,
}

impl ByteCodeStream {
    pub fn empty() -> Self {
        Self { code: vec![] }
    }

    pub fn from(code: Vec<Opcode>) -> Self {
        Self { code }
    }

    pub fn push(&mut self, opcode: Opcode) {
        self.code.push(opcode)
    }

    pub fn push_at(&mut self, opcode: Opcode, offset: usize) {
        if offset >= self.code.len() {
            panic!("Push opcode out of range offset");
        }
        self.code[offset] = opcode;
    }

    pub fn get_at(&self, offset: usize) -> Option<&Opcode> {
        if offset >= self.code.len() {
            return None;
        }
        return Some(&self.code[offset]);
    }

    pub fn get_pos(&self) -> usize {
        self.code.len()
    }

    pub fn clear(&mut self) {
        self.code.clear()
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nop => write!(f, "[Nop]"),
            Self::IAdd => write!(f, "[iadd]"),
            Self::Return => write!(f, "[return]"),
            Self::ReturnTop => write!(f, "[ireturn]"),
            Self::Invoke(name) => write!(f, "[invoke] {}", name),
            Self::LLoad(index) => write!(f, "[iload] {}", index),
            Self::LStore(index) => write!(f, "[istore] {}", index),
            Self::Push(iconst) => write!(f, "[bipush] {}", iconst),
            Self::IMul => write!(f, "[imul]"),
            Self::IDiv => write!(f, "[idiv]"),
            Self::Ldc(usize) => write!(f, "[ldc] {}", usize),
            Self::ICmpGT => write!(f, "[icmpgt]"),
            Self::JumpIfFalse(usize) => write!(f, "[jumpiffalse] {usize}"),
            Self::Jump(offset) => write!(f, "[jump] {offset}"),
        }
    }
}
