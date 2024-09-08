use crate::{frame::Frame, stack::Stack};
use bug::bytecode::{Opcode, PushOperand};
use bug::{stdlib::NativeFn, Program};
use bug::{DefinedFn, Object, Pool};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Engine {
  pool: Pool,
  frame: Frame,
  should_halt: bool,
  frame_stack: Stack<Frame>,
  // A set of built-in functions like `write`
  natives: HashMap<String, NativeFn>,
  // A set of user defined functions
  functions: HashMap<String, Rc<DefinedFn>>,
}

impl Engine {
  pub fn bootstrap(program: Program, natives: HashMap<String, NativeFn>) -> Self {
    let mut functions: HashMap<String, Rc<DefinedFn>> = HashMap::new();

    for (name, defined_fn) in program.fns {
      functions.insert(name, Rc::new(defined_fn));
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
      let op = match self.frame.fetch_next_op() {
        Some(op) => op.clone(),
        None => self.throw_fetch_out_of_range(),
      };

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
    let main = self.functions.get("main").unwrap_or_else(|| {
      self.throw_call_undefined("main");
    });
    self.frame = Frame::new("main".to_string(), Rc::clone(&main), main.max_locals);
  }

  fn engine_should_run(&self) -> bool {
    !self.should_halt
  }

  fn nop(&mut self) {
    return;
  }

  fn ldc(&mut self, idx: usize) {
    let o = self.pool.get_by_index(idx).unwrap_or_else(|| {
      self.throw_pool_index_out_of_range();
    });
    self.frame.push(o.clone());
  }

  fn invoke(&mut self, name: String) {
    if let Some(callee) = self.natives.get(&name) {
      let mut args: Vec<Object> = vec![];
      for _ in 0..callee.prototype.arity {
        let o = self.frame.pop().unwrap_or_else(|| {
          self.throw_stack_uderflow();
        });
        args.push(o);
      }
      if let Some(result) = (callee.function)(args) {
        self.frame.push(result);
      }
      return;
    }
    let callee = self.functions.get(&name).unwrap_or_else(|| {
      self.throw_call_undefined(&name);
    });
    let mut frame = Frame::new(name, Rc::clone(&callee), callee.max_locals);
    for idx in 0..callee.arity {
      let o = self.frame.pop().unwrap_or_else(|| {
        self.throw_stack_uderflow();
      });
      frame.store(callee.arity - idx - 1, o);
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
    if let Some(mut parent_frame) = self.frame_stack.pop() {
      match self.frame.pop() {
        Some(o) => parent_frame.push(o),
        _ => {}
      };
      self.frame = parent_frame;
    } else {
      self.should_halt = true;
    }
  }
}

// Exceptions

impl Engine {
  fn throw_pool_index_out_of_range(&self) -> ! {
    std::process::exit(1);
  }

  fn throw_fetch_out_of_range(&self) -> ! {
    eprintln!("RUNTIME EXCEPTION: Failed to fetch the next instruction");
    eprintln!("    At function `{}`", self.frame.get_name());
    std::process::exit(1);
  }

  fn throw_stack_uderflow(&self) -> ! {
    eprintln!("RUNTIME EXCEPTION: Stack underflow");
    eprintln!("    At function `{}`", self.frame.get_name());
    std::process::exit(1);
  }

  fn throw_call_undefined(&self, name: &str) -> ! {
    eprintln!("RUNTIME EXCEPTION: Call to undefined function `{name}`");
    eprintln!("    At function `{}`", self.frame.get_name());
    std::process::exit(1);
  }
}
