use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bytecode::Bytecode;
use crate::pool::Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub pool: Pool,
    pub fns: HashMap<String, DefinedFn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinedFn {
    pub arity: usize,
    pub code: Bytecode,
}
