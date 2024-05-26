use std::usize;

use crate::frame::Frame;
use crate::stack::Stack;
use spider_vm::bytecode::Opcode;
use spider_vm::object::Object;
use spider_vm::pool::PoolEntry;
use spider_vm::program::{Function, Program};

pub struct Runtime {
    program: Program,
    stack: Stack<Frame>,
}

impl Runtime {
    /// Setups the context needed for execution. Will load the function main, configures it's frame
    /// including operand-stack and all necessary information
    ///
    /// # Parameters
    ///
    /// - `program`: The structure that represents the user's program
    ///
    /// # Returns
    ///
    /// Will return a ready Runtime instance
    ///
    pub fn setup(program: Program) -> Self {
        let main_fn = program.fns[0].clone(); // `main` fn must be in the index 0
        let main_frame = Frame::make(main_fn.code, main_fn.max_locals, main_fn.max_stack);

        let mut stack = Stack::make(main_fn.max_stack);
        stack.push(main_frame);

        Self { program, stack }
    }

    /// Will start the execution of the program provided on `setup`
    /// the execution starts from main function
    ///
    pub fn run(&mut self) {
        // main frame
        let mut frame = self.stack.pop();

        // Main Loop
        // Note: All instruction that peform control flow such:
        // invoke, ireturn, return, jumps, ... must be handled inside of the main loop
        //
        loop {
            let instr = frame.fetch_next_instr();
            match instr {
                Opcode::IAdd => self.iadd(&mut frame),
                Opcode::IMul => self.imul(&mut frame),
                Opcode::IDiv => self.idiv(&mut frame),
                Opcode::ILdc(index) => self.ildc(index, &mut frame),
                Opcode::ILoad(index) => frame.opstack.push(frame.locals.get_by_index(index)),
                Opcode::IStore(index) => frame.locals.store_at(index, frame.opstack.pop()),
                Opcode::Invoke(index) => {
                    let callee = self.fn_loader(index);
                    let mut callee_frame =
                        Frame::make(callee.code, callee.max_locals, callee.max_stack);

                    for index in 0..callee.arity {
                        callee_frame.locals.store_at(index, frame.opstack.pop());
                    }

                    self.stack.push(frame.clone());
                    frame = callee_frame
                }
                Opcode::IReturn => {
                    let object_int = frame.stack_pop();
                    let mut outher = self.stack.pop();
                    outher.stack_push(object_int); // @TODO: check if object is int
                    frame = outher
                }
                Opcode::Goto(offset) => frame.pc = offset,
                Opcode::Return => {
                    if self.stack.is_empty() {
                        break;
                    }
                    frame = self.stack.pop();
                }
                Opcode::IfICmpE(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst == snd {
                        frame.pc = offset
                    }
                }
                Opcode::IfICmpNE(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst != snd {
                        frame.pc = offset
                    }
                }
                Opcode::IfICmpLT(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst < snd {
                        frame.pc = offset;
                    }
                }
                Opcode::IfICmpGT(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst > snd {
                        frame.pc = offset;
                    }
                }
                Opcode::IIncr(index, constant) => self.iincr(&mut frame, index, constant),
                Opcode::Bipush(iconst) => frame.opstack.push(Object::Int(iconst)),
            }
        }

        println!("{:#?}", frame.locals);
        println!("{:#?}", frame.opstack);
    }

    fn ipop_two(&mut self, frame: &mut Frame) -> (i32, i32) {
        let snd = match frame.stack_pop() {
            Object::Int(x) => x,
            _ => panic!("[ipop] expects int on stack"),
        };
        let fst = match frame.stack_pop() {
            Object::Int(y) => y,
            _ => panic!("[ipop] expects int on stack"),
        };

        (fst, snd)
    }

    fn iincr(&mut self, frame: &mut Frame, index: usize, constant: i32) {
        match frame.locals.get_as_ref(index) {
            Object::Int(x) => *x += constant,
            _ => panic!("[iincr] expects int"),
        };
    }

    fn iadd(&mut self, frame: &mut Frame) {
        let (lhs, rhs) = self.ipop_two(frame);
        frame.stack_push(Object::Int(lhs + rhs));
    }

    fn imul(&mut self, frame: &mut Frame) {
        let (lhs, rhs) = self.ipop_two(frame);
        frame.stack_push(Object::Int(lhs * rhs));
    }

    fn idiv(&mut self, frame: &mut Frame) {
        let (lhs, rhs) = self.ipop_two(frame);
        frame.stack_push(Object::Int(lhs / rhs));
    }

    /// Loads an integer object from pool to the operand stack
    /// panic if the entry in the provided index isn't the expected
    ///
    fn ildc(&mut self, index: usize, frame: &mut Frame) {
        match self.program.pool.get_by_index(index) {
            PoolEntry::Object(object) => match object {
                Object::Int(_) => frame.stack_push(object),
                _ => panic!("[ildc] expects int"),
            },
            _ => panic!("[ildc] expects int"),
        };
    }

    fn fn_loader(&self, pool_index: usize) -> Function {
        match self.program.pool.get_by_index(pool_index) {
            PoolEntry::FunctionRef(fn_ref) => self.program.load_fn(fn_ref.fn_index),
            _ => {
                panic!("fn_loader expects a `FunctionRef` at provided index");
            }
        }
    }
}
