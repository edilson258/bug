#[derive(Debug, Clone)]
pub struct Stack<T> {
    pub inner: Vec<T>,
}

impl<T: Clone + Sized> Stack<T> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn push(&mut self, frame: T) {
        self.inner.push(frame);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    /* pub fn is_empty(&self) -> bool {
      self.inner.is_empty()
    } */
}
