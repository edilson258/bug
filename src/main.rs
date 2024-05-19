mod bytecode;
mod core;
mod frame;
mod function;
mod object;
mod pool;
mod program;
mod stack;

use bytecode::{Bytecode, Instr};
use core::Runtime;
use function::Function;
use object::Object;
use pool::Pool;
use program::Program;

fn main() {
    let mut pool = Pool::make();
    let x = pool.append(Object::Int(34));
    let y = pool.append(Object::Int(35));

    let main_fn_instrs = vec![
        Instr::ILdc(x),
        Instr::ILdc(y),
        Instr::Invoke("f1".to_string()),
        Instr::IStore(0),
        Instr::Return,
    ];

    let f1_fn_instrs = vec![
        Instr::ILoad(0),
        Instr::ILoad(1),
        Instr::IAdd,
        Instr::Bipush(69),
        Instr::IfICmpE(6),
        Instr::Return,
        Instr::Invoke("f2".to_string()),
        Instr::IReturn,
        Instr::Goto(5),
    ];

    let f2_fn_instrs = vec![Instr::Bipush(1024), Instr::IReturn];

    let fns: Vec<Function> = vec![
        Function::make(
            "main".to_string(),
            0, // arity
            2, // max_stack
            1, // max_locals
            Bytecode::make(main_fn_instrs),
        ),
        Function::make(
            "f1".to_string(),
            2, // arity
            2, // max_stack
            2, // max_locals
            Bytecode::make(f1_fn_instrs),
        ),
        Function::make(
            "f2".to_string(),
            0, // arity
            1, // max_stack
            0, // max_locals
            Bytecode::make(f2_fn_instrs),
        ),
    ];

    let program = Program::make(pool, fns);
    let mut rt = Runtime::setup(program);
    rt.run();
}
