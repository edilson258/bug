use std::process::exit;

use crate::function::Function;
use crate::pool::Pool;

pub struct Program {
    pub pool: Pool,
    pub fns: Vec<Function>,
}

impl Program {
    pub fn make(pool: Pool, fns: Vec<Function>) -> Self {
        Self { pool, fns }
    }

    pub fn find_fn(&self, name: &str) -> Function {
        let rtn = self.fns.iter().find(|f| f.name.as_str() == name);
        if rtn.is_none() {
            eprintln!("[Error]: No function named {}", name);
            exit(1);
        }
        rtn.unwrap().clone()
    }
}
