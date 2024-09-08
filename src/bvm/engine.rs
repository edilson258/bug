use crate::function::{Function, Functions};
use crate::{frame::Frame, stack::Stack};
use bug::bytecode::{Opcode, PushOperand};
use bug::{stdlib::NativeFn, Program};
use bug::{Object, Pool};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Engine {
  pool: Pool,
  functions: Functions,
  frame: Frame,
  frame_stack: Stack<Frame>,
  natives: HashMap<String, NativeFn>,
  should_halt: bool,
}

impl Engine {
  pub fn bootstrap(program: Program, natives: HashMap<String, NativeFn>) -> Self {
    let mut functions: Functions = HashMap::new();

    for (name, f) in program.fns {
      functions.insert(name, Function::new(f.arity, f.max_locals, f.code));
    }

    Self {
      pool: program.pool,
      functions,
      frame: Frame::default(),
      frame_stack: Stack::new(),
      natives,
      should_halt: false,
    }
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
        Opcode::Push(operand) => self.push(operand),
        _ => unimplemented!(),
      };
    }
  }

  fn setup_main_frame(&mut self) {
    let main = self.functions.get("main").unwrap();
    self.frame = Frame::new(Rc::clone(&main.code), main.max_locals);
  }

  fn engine_should_run(&self) -> bool {
    !self.should_halt
  }

  fn nop(&mut self) {
    return;
  }

  fn ldc(&mut self, idx: usize) {
    self.frame.push(self.pool.get_by_index(idx));
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
    let callee = self.functions.get(&name).unwrap();
    let mut frame = Frame::new(Rc::clone(&callee.code), callee.max_locals);
    for idx in 0..callee.arity {
      frame.store(callee.arity - idx - 1, self.frame.pop());
    }
    self.frame_stack.push(self.frame.clone());
    self.frame = frame;
  }

  fn push(&mut self, operand: PushOperand) {
    match operand {
      PushOperand::Number(x) => self.frame.push(Object::Number(x)),
      PushOperand::Boolean(x) => self.frame.push(Object::Boolean(x)),
    };
  }

  fn return_(&mut self) {
    if let Some(parent_frame) = self.frame_stack.pop() {
      self.frame = parent_frame;
    } else {
      self.should_halt = true;
    }
  }
}
