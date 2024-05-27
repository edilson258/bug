use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Object {
    Null,
    Int(i32),
    String(String),
}

impl Default for Object {
    fn default() -> Self {
        Object::Null
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::Int(x) => write!(f, "{}", x),
            Self::String(x) => write!(f, "{}", x),
        }
    }
}
