use core::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Int(i32),
}

impl Default for Object {
    fn default() -> Self {
        Object::Null
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", x),
            Self::Null => write!(f, "NULL"),
        }
    }
}
