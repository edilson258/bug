use crate::stack::Stack;
use bug::bytecode::{ByteCodeStream, Opcode};
use bug::Object;

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

  pub fn get_at(&self, index: usize) -> Object {
    if index >= self.inner.len() {
      panic!("[Error]: Couldn't access to locals by index {}: OutOfRange", index);
    }
    self.inner[index].clone()
  }

  pub fn set_at(&mut self, index: usize, o: Object) {
    self.inner[index] = o;
  }
}

#[derive(Debug, Clone)]
pub struct Frame {
  pc: usize,
  locals: Locals,
  code: ByteCodeStream,
  stack: Stack<Object>,
}

impl Frame {
  pub fn new(code: ByteCodeStream, max_locals: usize) -> Self {
    Self { pc: 0, code, stack: Stack::new(), locals: Locals::new(max_locals) }
  }

  pub fn fetch_next_op(&mut self) -> &Opcode {
    let instr = self.code.get_at(self.pc).unwrap();
    self.pc += 1;
    instr
  }

  pub fn push(&mut self, o: Object) {
    self.stack.push(o);
  }

  pub fn pop(&mut self) -> Object {
    self.stack.pop().unwrap()
  }

  pub fn store(&mut self, idx: usize, o: Object) {
    self.locals.set_at(idx, o);
  }
}

impl Default for Frame {
  fn default() -> Self {
    Self { pc: 0, locals: Locals::new(0), code: ByteCodeStream::empty(), stack: Stack::new() }
  }
}
