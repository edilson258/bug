mod bytecode;
mod core;
mod frame;
mod function;
mod object;
mod pool;
mod program;
mod serde;
mod stack;

use bytecode::{Bytecode, Instr};
use core::Runtime;
use function::Function;
use object::Object;
use pool::Pool;
use program::Program;

fn main() {
    let mut pool = Pool::make();
    let _x = pool.append(Object::Int(34));
    let _y = pool.append(Object::Int(35));

    let instrs = vec![
        Instr::Bipush(0),
        Instr::IStore(0),
        Instr::Goto(4),
        Instr::IIncr(0, 1),
        Instr::ILoad(0),
        Instr::Bipush(1000000),
        Instr::IfICmpLT(3),
        Instr::Return,
    ];

    let fns: Vec<Function> = vec![
        Function::make(
            "main".to_string(),
            0, // arity
            2, // max_stack
            1, // max_locals
            Bytecode::make(instrs),
        ),
        Function::make(
            "sum".to_string(),
            2, // arity
            2, // max_stack
            2, // max_locals
            Bytecode::make(vec![
                Instr::ILoad(0),
                Instr::ILoad(1),
                Instr::IAdd,
                Instr::IReturn,
            ]),
        ),
    ];

    let program = Program::make(pool, fns);
    let mut rt = Runtime::setup(program);
    rt.run();
}
