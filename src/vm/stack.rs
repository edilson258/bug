use std::process::exit;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub inner: Vec<T>,
}

impl<T: Clone> Stack<T> {
    pub fn make() -> Self {
        Self { inner: vec![] }
    }

    pub fn push(&mut self, frame: T) {
        self.inner.push(frame);
    }

    pub fn pop(&mut self) -> T {
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
