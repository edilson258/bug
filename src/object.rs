use core::fmt;

pub const DEFAULT_OBJECT: Object = Object::Null;

#[derive(Debug, Clone)]
pub enum Object {
    Null,
    Int(i32),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(x) => write!(f, "{}", x),
            Self::Null => write!(f, "NULL"),
        }
    }
}
