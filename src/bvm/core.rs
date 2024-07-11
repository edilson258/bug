use crate::frame::Frame;
use crate::stack::Stack;

use bug::bytecode::{Opcode, PushOperand};
use bug::stdlib::list_native_fns;
use bug::{Object, PoolEntry, Program};

pub struct Runtime {}

impl Runtime {
    pub fn run(program: Program) {
        let native_fns = list_native_fns();
        let main_fn = program.fns.get("main").unwrap();
        let mut framestack: Stack<Frame> = Stack::make();
        let mut current_frame = Frame::make(main_fn.code.clone(), main_fn.max_locals);

        loop {
            let instr = current_frame.fetch_next_instr();
            match instr {
                Opcode::Nop => continue,
                Opcode::IAdd => Self::iadd(&mut current_frame),
                Opcode::IMul => Self::imul(&mut current_frame),
                Opcode::IDiv => Self::idiv(&mut current_frame),
                Opcode::LLoad(index) => current_frame
                    .stack
                    .push(current_frame.locals.get_by_index(index)),
                Opcode::LStore(index) => current_frame
                    .locals
                    .store_at(index, current_frame.stack.pop().unwrap()),
                Opcode::Invoke(name) => {
                    let defined_fn = program.fns.get(&name);
                    if defined_fn.is_some() {
                        let callee = defined_fn.unwrap().clone();
                        let mut callee_current_frame = Frame::make(callee.code, callee.max_locals);

                        for index in 0..callee.arity {
                            callee_current_frame.locals.store_at(
                                callee.arity - index - 1,
                                current_frame.stack.pop().unwrap(),
                            );
                        }

                        framestack.push(current_frame.clone());
                        current_frame = callee_current_frame
                    } else {
                        let native_fn = native_fns.get(&name).unwrap();
                        let mut args: Vec<Object> = vec![];
                        for _ in 0..native_fn.prototype.arity {
                            args.push(current_frame.stack.pop().unwrap());
                        }
                        if let Some(return_val) = (native_fn.function)(args) {
                            current_frame.stack.push(return_val);
                        }
                    }
                }
                Opcode::IReturn => {
                    let mut parent_frame = framestack.pop().unwrap();
                    parent_frame.stack.push(current_frame.stack.pop().unwrap());
                    current_frame = parent_frame;
                }
                Opcode::Return => {
                    if framestack.is_empty() {
                        break;
                    }
                    current_frame = framestack.pop().unwrap();
                }
                Opcode::ICmpGT => {
                    let (lhs, rhs) = Self::ipop_two(&mut current_frame);
                    if lhs > rhs {
                        current_frame.stack.push(Object::Boolean(true));
                    } else {
                        current_frame.stack.push(Object::Boolean(false));
                    }
                }
                Opcode::JumpIfFalse(offset) => {
                    let val = match current_frame.stack.pop().unwrap() {
                        Object::Boolean(val) => val,
                        unexpected => panic!(
                            "Expected boolean, got {} for 'JumpIfFalse' opcode",
                            unexpected
                        ),
                    };
                    if val == false {
                        current_frame.pc = offset;
                    }
                }
                Opcode::Push(val) => match val {
                    PushOperand::Integer(x) => {
                        current_frame.stack.push(Object::Int(x));
                    }
                    PushOperand::Boolean(x) => {
                        current_frame.stack.push(Object::Boolean(x));
                    }
                },
                Opcode::Ldc(index) => match program.pool.entries[index] {
                    PoolEntry::Object(ref object) => current_frame.stack.push(object.clone()),
                },
            }
        }
    }

    fn ipop_two(current_frame: &mut Frame) -> (i32, i32) {
        let snd = match current_frame.stack.pop().unwrap() {
            Object::Int(x) => x,
            _ => panic!("[ipop] expects int on stack"),
        };
        let fst = match current_frame.stack.pop().unwrap() {
            Object::Int(y) => y,
            _ => panic!("[ipop] expects int on stack"),
        };

        (fst, snd)
    }

    fn iadd(current_frame: &mut Frame) {
        let (lhs, rhs) = Self::ipop_two(current_frame);
        current_frame.stack.push(Object::Int(lhs + rhs));
    }

    fn imul(current_frame: &mut Frame) {
        let (lhs, rhs) = Self::ipop_two(current_frame);
        current_frame.stack.push(Object::Int(lhs * rhs));
    }

    fn idiv(current_frame: &mut Frame) {
        let (lhs, rhs) = Self::ipop_two(current_frame);
        current_frame.stack.push(Object::Int(lhs / rhs));
    }
}
