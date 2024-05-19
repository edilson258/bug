use std::process::exit;

pub const MAX_STACK: usize = 5;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub sp: usize,
    pub max_stack: usize,
    pub inner: [T; MAX_STACK],
}

impl<T: Default + Clone> Stack<T> {
    pub fn make(cap: usize) -> Self {
        if cap > MAX_STACK {
            eprintln!("[Error]: Cannot have stack of size {}: OutOfBound", cap);
            exit(1);
        }
        Self {
            sp: 0,
            max_stack: cap,
            inner: Default::default(),
        }
    }

    pub fn push(&mut self, frame: T) {
        if self.sp >= self.max_stack {
            eprintln!("[Error]: Couldn't push onto stack: StackOverFlow");
            exit(1);
        }
        self.inner[self.sp] = frame;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> T {
        if self.sp <= 0 {
            eprintln!("[Error]: Couldn't pop from stack: StackUnderFlow");
            exit(1);
        }
        self.sp -= 1;
        self.inner[self.sp].clone()
    }

    pub fn is_empty(&self) -> bool {
        self.sp <= 0
    }
}
