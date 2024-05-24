mod bytecode;
mod core;
mod frame;
mod object;
mod pool;
mod program;
mod stack;

use bytecode::{Bytecode, Opcode};
use core::Runtime;
use object::Object;
use pool::{FunctionRef, Pool, PoolEntry};
use program::{Function, Program};

fn main() {
    let mut pool = Pool::make();
    let x = pool.append(PoolEntry::Object(Object::Int(34)));
    let y = pool.append(PoolEntry::Object(Object::Int(35)));

    /*
     * Function2 setup
     */
    let f2_name_index = pool.append(PoolEntry::Object(Object::String("fn1".to_string())));
    let f2_sign_index = pool.append(PoolEntry::Object(Object::String("(II)I".to_string())));
    let f2_ref = pool.append(PoolEntry::FunctionRef(FunctionRef::make(
        f2_name_index,
        f2_sign_index,
        2,
    )));
    let f2_instrs = vec![Opcode::Bipush(1024), Opcode::IReturn];

    /*
     * Function1 setup
     */
    let f1_name_index = pool.append(PoolEntry::Object(Object::String("fn1".to_string())));
    let f1_sign_index = pool.append(PoolEntry::Object(Object::String("(II)I".to_string())));
    let f1_ref = pool.append(PoolEntry::FunctionRef(FunctionRef::make(
        f1_name_index,
        f1_sign_index,
        1,
    )));
    let f1_instrs = vec![
        Opcode::ILoad(0),
        Opcode::ILoad(1),
        Opcode::IAdd,
        Opcode::Bipush(69),
        Opcode::IfICmpE(6),
        Opcode::Return,
        Opcode::Invoke(f2_ref),
        Opcode::IReturn,
        Opcode::Goto(5),
    ];

    /*
     * Main function setup
     */
    let main_name_index = pool.append(PoolEntry::Object(Object::String("main".to_string())));
    let main_sign_index = pool.append(PoolEntry::Object(Object::String("()V".to_string())));
    let main_ref = pool.append(PoolEntry::FunctionRef(FunctionRef::make(
        main_name_index,
        main_sign_index,
        0, // main must be in pos 0 in functions list
    )));
    let main_instrs = vec![
        Opcode::ILdc(x),
        Opcode::ILdc(y),
        Opcode::Invoke(f1_ref),
        Opcode::IStore(0),
        Opcode::Return,
    ];

    let fns: Vec<Function> = vec![
        Function::make(
            main_ref, // pool ref
            0,        // arity
            2,        // max_stack
            1,        // max_locals
            Bytecode::make(main_instrs),
        ),
        Function::make(
            f1_ref,
            2, // arity
            2, // max_stack
            2, // max_locals
            Bytecode::make(f1_instrs),
        ),
        Function::make(
            f2_ref,
            0, // arity
            1, // max_stack
            0, // max_locals
            Bytecode::make(f2_instrs),
        ),
    ];

    let program = Program::make(pool, fns);
    let mut rt = Runtime::setup(program);
    rt.run();
}
