#[derive(Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(line: usize, column: usize, start: usize, end: usize) -> Self {
        Self { line, column, start, end }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self { line: 0, column: 0, start: 0, end: 0 }
    }
}
