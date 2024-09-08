use bug::bytecode::ByteCodeStream;
use std::{collections::HashMap, rc::Rc};

/**
 *
 * Same as `DefinedFn` but with the `ByteCodeStream` wrapped in `Rc`
 *
 */
pub struct Function {
  pub arity: usize,
  pub max_locals: usize,
  pub code: Rc<ByteCodeStream>,
}

impl Function {
  pub fn new(arity: usize, max_locals: usize, code: ByteCodeStream) -> Self {
    Self { arity, max_locals, code: Rc::new(code) }
  }
}

pub type Functions = HashMap<String, Function>;
