use crate::{frame::Frame, stack::Stack};
use bug::bytecode::Opcode;
use bug::Object;
use bug::{stdlib::NativeFn, Program};
use std::collections::HashMap;

pub struct Engine {
  program: Program,
  frame: Frame,
  frame_stack: Stack<Frame>,
  natives: HashMap<String, NativeFn>,
  should_halt: bool,
}

impl Engine {
  pub fn bootstrap(program: Program, natives: HashMap<String, NativeFn>) -> Self {
    Self { program, frame: Frame::default(), frame_stack: Stack::make(), natives, should_halt: false }
  }

  pub fn run(&mut self) {
    self.setup_main_frame();

    while self.engine_should_run() {
      let op = self.frame.fetch_next_op().clone();
      match op {
        Opcode::Nop => self.nop(),
        Opcode::Return => self.return_(),
        Opcode::Ldc(idx) => self.ldc(idx),
        Opcode::Invoke(name) => self.invoke(name),
        _ => unimplemented!(),
      };
    }
  }

  fn setup_main_frame(&mut self) {
    let main = self.program.fns.get("main").unwrap();
    self.frame = Frame::make(main.code.clone() /* Don't like this clone */, main.max_locals);
  }

  fn engine_should_run(&self) -> bool {
    !self.should_halt
  }

  fn nop(&mut self) {
    return;
  }

  fn ldc(&mut self, idx: usize) {
    self.frame.push(self.program.pool.get_by_index(idx));
  }

  fn invoke(&mut self, name: String) {
    if let Some(callee) = self.natives.get(&name) {
      let mut args: Vec<Object> = vec![];
      for _ in 0..callee.prototype.arity {
        args.push(self.frame.pop())
      }
      if let Some(result) = (callee.function)(args) {
        self.frame.push(result);
      }
      return;
    }
    let callee = self.program.fns.get(&name).unwrap();
    let mut frame = Frame::make(callee.code.clone() /* Don't like this clone */, callee.max_locals);
    for idx in 0..callee.arity {
      frame.store(callee.arity - idx - 1, self.frame.pop());
    }
    self.frame_stack.push(self.frame.clone());
    self.frame = frame;
  }

  fn return_(&mut self) {
    if let Some(parent_frame) = self.frame_stack.pop() {
      self.frame = parent_frame;
    } else {
      self.should_halt = true;
    }
  }
}
