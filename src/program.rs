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
