use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::bytecode::Bytecode;
use crate::pool::Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub pool: Pool,
    pub fns: HashMap<String, Function>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub arity: usize,
    pub code: Bytecode,
}
