use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Opcode {
    /// Will do nothing for a cycle
    NOP,
    /// Add two ints on top of the stack and push the result
    IADD,
    /// Will pop and compare the topest ints on the stack and then push true if the lhs is grather
    /// than the rhs otherwise false
    ICMPGT,
    /// Will return from current function
    RETURN,
    /// Returns the value on the top of the current stack
    INVOKE(String),
    // Will load some Object from pool and push on the stack
    LDC(usize),
    /// Will load a value from locals at provided index to the stack
    LLOAD(usize),
    /// Will move a value from top of the stack to the locals at provided index
    LSTORE(usize),
    /// Will push an imediate integer value to the stack
    IPUSH(i32),
    JUMP(usize),
    JUMPNOTIF(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteCodeStream {
    pub code: Vec<Opcode>,
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

    pub fn push_at(&mut self, opcode: Opcode, offset: usize) -> bool {
        if offset >= self.code.len() {
            return false;
        }
        self.code[offset] = opcode;
        true
    }

    pub fn get_at(&self, offset: usize) -> Option<&Opcode> {
        self.code.get(offset)
    }

    pub fn get_pos(&self) -> usize {
        self.code.len()
    }

    pub fn clear(&mut self) {
        self.code.clear()
    }
}
