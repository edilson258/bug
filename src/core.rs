use crate::bytecode::Instr;
use crate::frame::Frame;
use crate::object::Object;
use crate::program::Program;
use crate::stack::Stack;

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
        let main_fn = program.find_fn("main");
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
                Instr::IAdd => self.iadd(&mut frame),
                Instr::IMul => self.imul(&mut frame),
                Instr::IDiv => self.idiv(&mut frame),
                Instr::ILdc(index) => self.ildc(index, &mut frame),
                Instr::ILoad(index) => frame.opstack.push(frame.locals.get_by_index(index)),
                Instr::IStore(index) => frame.locals.store_at(index, frame.opstack.pop()),
                Instr::Invoke(name) => {
                    let callee = self.program.find_fn(&name);
                    let mut callee_frame =
                        Frame::make(callee.code, callee.max_locals, callee.max_stack);

                    for index in 0..callee.arity {
                        callee_frame.locals.store_at(index, frame.opstack.pop());
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
                Instr::Goto(offset) => frame.pc = offset,
                Instr::Return => {
                    if self.stack.is_empty() {
                        break;
                    }
                    frame = self.stack.pop();
                }
                Instr::IfICmpE(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst == snd {
                        frame.pc = offset
                    }
                }
                Instr::IfICmpNE(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst != snd {
                        frame.pc = offset
                    }
                }
                Instr::IfICmpLT(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst < snd {
                        frame.pc = offset;
                    }
                }
                Instr::IfICmpGT(offset) => {
                    let (fst, snd) = self.ipop_two(&mut frame);
                    if fst > snd {
                        frame.pc = offset;
                    }
                }
                Instr::IIncr(index, constant) => self.iincr(&mut frame, index, constant),
                Instr::Bipush(iconst) => frame.opstack.push(Object::Int(iconst)),
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
            _ => panic!("[iincr] expects an int"),
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

    fn ildc(&mut self, index: usize, frame: &mut Frame) {
        let x = self.program.pool.get_by_index(index);
        frame.stack_push(x);
    }
}
