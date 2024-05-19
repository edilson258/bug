use std::process::exit;

use crate::{
    frame::Frame,
    object::{Object, DEFAULT_OBJECT},
};

const MAX_STACK: usize = 25;

#[derive(Debug, Clone)]
pub struct OpStack {
    pub sp: usize,
    pub max_stack: usize,
    pub inner: [Object; MAX_STACK],
}

impl OpStack {
    pub fn make(cap: usize) -> Self {
        if cap > MAX_STACK {
            eprintln!("[Error]: Cannot have stack of size {}: OutOfBound", cap);
            exit(1);
        }
        Self {
            sp: 0,
            max_stack: cap,
            inner: [DEFAULT_OBJECT; MAX_STACK],
        }
    }

    pub fn push(&mut self, item: Object) {
        if self.sp >= self.max_stack {
            eprintln!("[Error]: Couldn't push onto stack: StackOverFlow");
            exit(1);
        }
        self.inner[self.sp] = item;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> Object {
        if self.sp <= 0 {
            eprintln!("[Error]: Couldn't pop from stack: StackUnderFlow");
            exit(1);
        }
        self.sp -= 1;
        self.inner[self.sp].clone()
    }
}

#[derive(Debug)]
pub struct FrameStack {
    max_stack: usize,
    inner: Vec<Frame>,
}

impl FrameStack {
    pub fn make(cap: usize) -> Self {
        if cap > MAX_STACK {
            eprintln!("[Error]: Cannot have stack of size {}: OutOfBound", cap);
            exit(1);
        }
        Self {
            max_stack: cap,
            inner: vec![],
        }
    }

    pub fn push(&mut self, item: Frame) {
        if self.inner.len() >= self.max_stack {
            eprintln!("[Error]: Couldn't push onto stack: StackOverFlow");
            exit(1);
        }
        self.inner.push(item);
    }

    pub fn pop(&mut self) -> Frame {
        if self.inner.is_empty() {
            eprintln!("[Error]: Couldn't pop from stack: StackUnderFlow");
            exit(1);
        }
        self.inner.pop().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
