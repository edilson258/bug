use crate::stack::Stack;
use bug::bytecode::Opcode;
use bug::{DefinedFn, Object};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Locals {
  inner: Vec<Object>,
}

impl Locals {
  pub fn new(max_locals: usize) -> Self {
    let mut inner: Vec<Object> = Vec::with_capacity(max_locals);
    for _ in 0..max_locals {
      inner.push(Object::Number(0.)) // init locals with zeros
    }
    Self { inner }
  }

  pub fn get_at(&self, index: usize) -> Option<&Object> {
    self.inner.get(index)
  }

  pub fn set_at(&mut self, index: usize, o: Object) {
    self.inner[index] = o;
  }
}

#[derive(Debug, Clone)]
pub struct Frame {
  pc: usize,
  locals: Locals,
  name: String,
  function: Rc<DefinedFn>,
  stack: Stack<Object>,
}

impl Frame {
  pub fn new(name: String, function: Rc<DefinedFn>, max_locals: usize) -> Self {
    Self { pc: 0, name, function, stack: Stack::new(), locals: Locals::new(max_locals) }
  }

  pub fn fetch_next_op(&mut self) -> Option<&Opcode> {
    let instr = self.function.code.get_at(self.pc);
    self.pc += 1;
    instr
  }

  pub fn push(&mut self, o: Object) {
    self.stack.push(o);
  }

  pub fn pop(&mut self) -> Option<Object> {
    self.stack.pop()
  }

  pub fn store(&mut self, idx: usize, o: Object) {
    self.locals.set_at(idx, o);
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }
}

impl Default for Frame {
  fn default() -> Self {
    Self {
      pc: 0,
      name: String::new(),
      locals: Locals::new(0),
      function: Rc::new(DefinedFn::default()),
      stack: Stack::new(),
    }
  }
}
