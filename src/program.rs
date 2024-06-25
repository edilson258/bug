use std::process::exit;

use serde::{Deserialize, Serialize};

use crate::bytecode::Bytecode;
use crate::pool::Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub pool: Pool,
    pub fns: Vec<Function>,
}

impl Program {
    pub fn make(pool: Pool, fns: Vec<Function>) -> Self {
        Self { pool, fns }
    }

    /// Gets a function from list of function and panic if index is out of range
    ///
    /// # Parameters
    /// - `index`: The index obtained on `append_fn`
    ///
    /// # Return
    /// - `Function`: A function in the provided index
    ///
    pub fn load_fn(&self, index: usize) -> Function {
        if index >= self.fns.len() {
            eprintln!("[Error]: Unable to get function: IndexOutOfRange");
            exit(1);
        }

        self.fns[index].clone()
    }

    /// Appends a function to the list of functions
    ///
    /// # Parameters
    /// - `function`: an instance of `Function`
    ///
    /// # Return
    /// - `index`: of within the list of functions
    ///
    pub fn append_fn(&mut self, func: Function) -> usize {
        let index = self.fns.len();
        self.fns.push(func);
        index
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub code: Bytecode,
    pub max_stack: usize,
    pub max_locals: usize,
}

impl Function {
    pub fn make(
        name: String,
        arity: usize,
        max_stack: usize,
        max_locals: usize,
        code: Bytecode,
    ) -> Self {
        Self {
            name,
            arity,
            code,
            max_stack,
            max_locals,
        }
    }
}
