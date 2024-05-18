use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
enum Object {
    Int(i32),
}

struct Pool {
    entries: Vec<Object>,
}

impl Pool {
    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn append(&mut self, o: Object) -> usize {
        let index = self.entries.len();
        self.entries.push(o);
        index
    }

    fn get_by_index(&mut self, i: usize) -> Object {
        if self.entries.len() <= i {
            panic!("[Error]: Pool out of range")
        }
        self.entries[i].clone()
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    ILdc(usize),
    IStore(usize),
    ILoad(usize),
    Invoke(String),

    IAdd,
    IReturn,
    Retrun,
}

#[derive(Debug, Clone)]
struct Bytecode {
    instructions: Vec<Instruction>,
}

impl Bytecode {
    pub fn fetch(&self, i: usize) -> Instruction {
        if i >= self.instructions.len() {
            panic!("No Instruction to fetch")
        }
        self.instructions[i].clone()
    }
}

#[derive(Debug)]
struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    fn pop(&mut self) -> T {
        if self.inner.is_empty() {
            panic!("[Error]: StackUnderFlow")
        }
        self.inner.pop().unwrap()
    }
}

type FrameParent = Option<Rc<RefCell<Frame>>>;

struct Frame {
    locals: Vec<Object>,
    opstack: Stack<Object>,
    code: Bytecode,
    parent: FrameParent,
    pc: usize,
}

impl Frame {
    fn from_fn(m: Function, parent: FrameParent) -> Self {
        Frame {
            pc: 0,
            parent,
            code: m.code,
            locals: Vec::new(),
            opstack: Stack::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    code: Bytecode,
}

struct Program {
    pool: Pool,
    fns: Vec<Function>,
}

impl Program {
    fn new(pool: Pool, fns: Vec<Function>) -> Self {
        Self { pool, fns }
    }

    fn find_fn(&self, name: &str) -> Function {
        let rtn = self.fns.iter().find(|f| f.name.as_str() == name);
        if rtn.is_none() {
            panic!("Missing main function");
        }
        rtn.unwrap().clone()
    }
}

struct Runtime {
    program: Program,
    frame: Rc<RefCell<Frame>>,
}

impl Runtime {
    fn setup(program: Program) -> Self {
        let main_fn = program.find_fn("main");
        Self {
            program,
            frame: Rc::new(RefCell::new(Frame::from_fn(main_fn, None))),
        }
    }

    fn run(&mut self) {
        loop {
            let instr = self.frame.borrow().code.fetch(self.frame.borrow().pc);
            self.frame.borrow_mut().pc += 1;
            match instr {
                Instruction::IReturn => {
                    let int = self.frame.borrow_mut().opstack.pop();
                    let parent_frame = self.frame.borrow().parent.clone().unwrap();
                    self.frame = parent_frame;
                    self.frame.borrow_mut().opstack.push(int);
                    break;
                }
                Instruction::Retrun => break,
                Instruction::ILoad(index) => self.iload(index),
                Instruction::IStore(index) => self.istore(index),
                Instruction::IAdd => self.iadd(),
                Instruction::Invoke(name) => self.invoke(&name),
                Instruction::ILdc(index) => self.ldc(index),
            };
        }
    }

    fn iload(&mut self, index: usize) {
        let o = self.frame.borrow().locals[index].clone();
        self.frame.borrow_mut().opstack.push(o);
    }

    fn istore(&mut self, _index: usize) {
        let o = self.frame.borrow_mut().opstack.pop();
        self.frame.borrow_mut().locals.push(o);
    }

    fn iadd(&mut self) {
        let lhs = match self.frame.borrow_mut().opstack.pop() {
            Object::Int(x) => x,
        };
        let rhs = match self.frame.borrow_mut().opstack.pop() {
            Object::Int(x) => x,
        };
        self.frame.borrow_mut().opstack.push(Object::Int(lhs + rhs));
    }

    fn invoke(&mut self, name: &str) {
        let func = self.program.find_fn(name);
        let mut func_frame = Frame::from_fn(func, Some(Rc::clone(&self.frame)));
        func_frame
            .locals
            .push(self.frame.borrow_mut().opstack.pop());
        func_frame
            .locals
            .push(self.frame.borrow_mut().opstack.pop());
        self.frame = Rc::new(RefCell::new(func_frame));
        self.run();
    }

    fn ldc(&mut self, index: usize) {
        self.frame
            .borrow_mut()
            .opstack
            .push(self.program.pool.get_by_index(index));
    }
}

fn main() {
    let mut pool = Pool::new();
    let x = pool.append(Object::Int(34));
    let y = pool.append(Object::Int(35));

    let sum_fn = Function {
        name: "sum".to_string(),
        code: Bytecode {
            instructions: vec![
                Instruction::ILoad(0),
                Instruction::ILoad(1),
                Instruction::IAdd,
                Instruction::IReturn,
            ],
        },
    };
    let main_fn = Function {
        name: "main".to_string(),
        code: Bytecode {
            instructions: vec![
                Instruction::ILdc(x),
                Instruction::ILdc(y),
                Instruction::Invoke("sum".to_string()),
                Instruction::IStore(0),
                Instruction::Retrun,
            ],
        },
    };
    let fns: Vec<Function> = vec![main_fn, sum_fn];

    let program = Program::new(pool, fns);
    let mut rt = Runtime::setup(program);
    rt.run();

    let main_fn_stack = &rt.frame.borrow().opstack;
    let main_fn_locals = &rt.frame.borrow().locals;
    println!("Stack: {:#?}", main_fn_stack);
    println!("Locals: {:#?}", main_fn_locals);
}
