use crate::frame::Frame;
use crate::stack::Stack;
use spider_vm::bytecode::Opcode;
use spider_vm::object::Object;
use spider_vm::pool::PoolEntry;
use spider_vm::program::Program;
use spider_vm::std::list_builtin_fns;

pub struct Runtime {}

impl Runtime {
    pub fn run(program: Program) {
        let program = program;
        let native_fns = list_builtin_fns();
        let main_fn = program.fns.iter().find(|func| func.name == "main").unwrap();
        let mut framestack: Stack<Frame> = Stack::make();
        let mut current_frame = Frame::make(main_fn.code.clone());

        // Main Loop
        // Note: All instruction that peform control flow such:
        // invoke, ireturn, return, jumps, ... must be handled inside of the main loop
        //
        loop {
            let instr = current_frame.fetch_next_instr();
            match instr {
                Opcode::IAdd => Self::iadd(&mut current_frame),
                Opcode::IMul => Self::imul(&mut current_frame),
                Opcode::IDiv => Self::idiv(&mut current_frame),
                Opcode::ILdc(index) => {
                    match program.pool.get_by_index(index) {
                        PoolEntry::Object(object) => match object {
                            Object::Int(_) => current_frame.stack_push(object),
                            _ => panic!("[ildc] expects int"),
                        },
                    };
                }
                Opcode::ILoad(index) => current_frame
                    .opstack
                    .push(current_frame.locals.get_by_index(index)),
                Opcode::IStore(index) => current_frame
                    .locals
                    .store_at(index, current_frame.opstack.pop()),
                Opcode::Invoke(name) => {
                    let func = program.fns.iter().find(|func| func.name == name);
                    if func.is_some() {
                        let callee = func.unwrap().clone();
                        let mut callee_current_frame = Frame::make(callee.code);

                        for index in 0..callee.arity {
                            callee_current_frame
                                .locals
                                .store_at(index, current_frame.opstack.pop());
                        }

                        framestack.push(current_frame.clone());
                        current_frame = callee_current_frame
                    } else {
                        let native_fn = native_fns
                            .iter()
                            .find(|native_fn| native_fn.name == name)
                            .unwrap();
                        let mut args: Vec<Object> = vec![];
                        for _ in 0..native_fn.prototype.arity {
                            args.push(current_frame.opstack.pop());
                        }
                        (native_fn.function)(args);
                    }
                }
                Opcode::IReturn => {
                    let object_int = current_frame.stack_pop();
                    let mut outher = framestack.pop();
                    outher.stack_push(object_int); // @TODO: check if object is int
                    current_frame = outher
                }
                Opcode::Goto(offset) => current_frame.pc = offset,
                Opcode::Return => {
                    if framestack.is_empty() {
                        break;
                    }
                    current_frame = framestack.pop();
                }
                Opcode::IfICmpE(offset) => {
                    let (fst, snd) = Self::ipop_two(&mut current_frame);
                    if fst == snd {
                        current_frame.pc = offset
                    }
                }
                Opcode::IfICmpNE(offset) => {
                    let (fst, snd) = Self::ipop_two(&mut current_frame);
                    if fst != snd {
                        current_frame.pc = offset
                    }
                }
                Opcode::IfICmpLT(offset) => {
                    let (fst, snd) = Self::ipop_two(&mut current_frame);
                    if fst < snd {
                        current_frame.pc = offset;
                    }
                }
                Opcode::IfICmpGT(offset) => {
                    let (fst, snd) = Self::ipop_two(&mut current_frame);
                    if fst > snd {
                        current_frame.pc = offset;
                    }
                }
                Opcode::IIncr(index, constant) => Self::iincr(&mut current_frame, index, constant),
                Opcode::Bipush(iconst) => current_frame.opstack.push(Object::Int(iconst)),
                Opcode::Ldc(_) => todo!(),
            }
        }
    }

    fn ipop_two(current_frame: &mut Frame) -> (i32, i32) {
        let snd = match current_frame.stack_pop() {
            Object::Int(x) => x,
            _ => panic!("[ipop] expects int on stack"),
        };
        let fst = match current_frame.stack_pop() {
            Object::Int(y) => y,
            _ => panic!("[ipop] expects int on stack"),
        };

        (fst, snd)
    }

    fn iincr(current_frame: &mut Frame, index: usize, constant: i32) {
        match current_frame.locals.get_as_ref(index) {
            Object::Int(x) => *x += constant,
            _ => panic!("[iincr] expects int"),
        };
    }

    fn iadd(current_frame: &mut Frame) {
        let (lhs, rhs) = Self::ipop_two(current_frame);
        current_frame.stack_push(Object::Int(lhs + rhs));
    }

    fn imul(current_frame: &mut Frame) {
        let (lhs, rhs) = Self::ipop_two(current_frame);
        current_frame.stack_push(Object::Int(lhs * rhs));
    }

    fn idiv(current_frame: &mut Frame) {
        let (lhs, rhs) = Self::ipop_two(current_frame);
        current_frame.stack_push(Object::Int(lhs / rhs));
    }
}
