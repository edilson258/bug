use crate::{frame::Frame, stack::Stack};
use bug::bytecode::Opcode;
use bug::{stdlib::NativeFn, Program};
use bug::{DefinedFn, Object, Pool};
use std::collections::HashMap;
use std::ops::Add;
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
                Opcode::NOP => self.nop(),
                Opcode::IADD => self.iadd(),
                Opcode::RETURN => self.return_(),
                Opcode::LDC(idx) => self.ldc(idx),
                Opcode::LLOAD(idx) => self.lload(idx),
                Opcode::INVOKE(name) => self.invoke(name),
                Opcode::IPUSH(integer) => self.ipush(integer),
                Opcode::ICMPGT => self.icmpgt(),
                Opcode::JUMP(offset) => self.jump(offset),
                Opcode::JUMPNOTIF(offset) => self.jumpnotif(offset),
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

    fn iadd(&mut self) {
        let rhs_integer = match self.frame.pop() {
            Some(o) => match o {
                Object::Integer(integer) => integer,
                _ => unreachable!(),
            },
            None => self.throw_stack_uderflow(),
        };
        let lhs_integer = match self.frame.pop() {
            Some(o) => match o {
                Object::Integer(integer) => integer,
                _ => unreachable!(),
            },
            None => self.throw_stack_uderflow(),
        };
        self.frame.push(Object::Integer(lhs_integer.add(rhs_integer)));
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

    fn return_(&mut self) {
        if let Some(mut parent_frame) = self.frame_stack.pop() {
            match self.frame.pop() {
                Some(o) => parent_frame.push(o),
                None => {}
            };
            self.frame = parent_frame;
        } else {
            // We got return on the main function
            self.should_halt = true;
        }
    }

    fn ipush(&mut self, integer: i32) {
        self.frame.push(Object::Integer(integer));
    }

    fn lload(&mut self, idx: usize) {
        let o = self.frame.load(idx).unwrap().clone();
        self.frame.push(o);
    }

    fn icmpgt(&mut self) {
        let rhs = match self.frame.pop() {
            Some(o) => match o {
                Object::Integer(integer) => integer,
                _ => unreachable!(),
            },
            None => self.throw_stack_uderflow(),
        };
        let lhs = match self.frame.pop() {
            Some(o) => match o {
                Object::Integer(integer) => integer,
                _ => unreachable!(),
            },
            None => self.throw_stack_uderflow(),
        };
        if lhs > rhs {
            self.frame.push(Object::Boolean(true))
        } else {
            self.frame.push(Object::Boolean(false))
        }
    }

    fn jump(&mut self, offset: usize) {
        self.frame.ip = offset;
    }

    fn jumpnotif(&mut self, offset: usize) {
        let condition_obj = match self.frame.pop() {
            Some(o) => o,
            None => self.throw_stack_uderflow(),
        };
        let condition = match condition_obj {
            Object::Boolean(b) => b,
            _ => unreachable!(),
        };
        // jump if not will jump if the condition is false
        if condition == false {
            self.frame.ip = offset;
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
