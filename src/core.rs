use crate::{bytecode::Instr, frame::Frame, object::Object, program::Program, stack::FrameStack};

pub struct Runtime {
    program: Program,
    stack: FrameStack,
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
        let main_fn = program.find_fn("main");
        let main_frame = Frame::make(main_fn.code, main_fn.max_locals, main_fn.max_stack);

        let mut stack = FrameStack::make(2);
        stack.push(main_frame);

        Self { program, stack }
    }

    /// Will start the execution of the program provided on `setup`
    /// the execution starts from main function
    ///
    pub fn run(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        let mut frame = self.stack.pop();

        // Main Loop
        loop {
            let instr = frame.fetch_next_instr();
            println!("{:?}", instr);
            match instr {
                Instr::IAdd => self.iadd(&mut frame),
                Instr::ILdc(index) => self.ildc(index, &mut frame),
                Instr::ILoad(index) => self.iload(index, &mut frame),
                Instr::IStore(index) => self.istore(index, &mut frame),
                Instr::Invoke(name) => {
                    let callee = self.program.find_fn(&name);
                    let mut callee_frame =
                        Frame::make(callee.code, callee.max_locals, callee.max_stack);

                    for _ in 0..callee.arity {
                        let x = frame.stack_pop();
                        callee_frame.locals_append(x);
                    }

                    self.stack.push(frame.clone());
                    frame = callee_frame
                }
                Instr::IReturn => {
                    let x = frame.stack_pop();
                    let mut outher = self.stack.pop();
                    outher.stack_push(x);
                    frame = outher
                }
                Instr::Retrun => break,
            }
        }

        println!("{:#?}", frame);
    }

    fn iadd(&mut self, frame: &mut Frame) {
        let lhs = match frame.stack_pop() {
            Object::Int(x) => x,
            _ => panic!("[iadd] expects int on the stack"),
        };
        let rhs = match frame.stack_pop() {
            Object::Int(x) => x,
            _ => panic!("[iadd] expects int on the stack"),
        };
        frame.stack_push(Object::Int(lhs + rhs));
    }

    fn ildc(&mut self, index: usize, frame: &mut Frame) {
        let x = self.program.pool.get_by_index(index);
        frame.stack_push(x);
    }

    fn iload(&mut self, index: usize, frame: &mut Frame) {
        let x = frame.locals_get_by_index(index);
        frame.stack_push(x);
    }

    fn istore(&mut self, _index: usize, frame: &mut Frame) {
        let x = frame.stack_pop();
        frame.locals_append(x);
    }
}
