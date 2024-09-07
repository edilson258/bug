pub mod bytecode;
pub mod stdlib;
pub mod utils;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use bytecode::ByteCodeStream;
use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
  Void,
  String,
  Integer,
  Boolean,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Void => write!(f, "void"),
      Self::String => write!(f, "str"),
      Self::Integer => write!(f, "int"),
      Self::Boolean => write!(f, "bool"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct FnPrototype {
  pub arity: u8,
  pub argtypes: Vec<Type>,
  pub return_type: Type,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Object {
  Number(f32),
  String(String),
  Boolean(bool),
}

impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Number(x) => write!(f, "{}", x),
      Self::String(x) => write!(f, "{}", x),
      Self::Boolean(x) => write!(f, "{}", x),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
  pub pool: Pool,
  pub fns: HashMap<String, DefinedFn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinedFn {
  pub arity: usize,
  pub code: ByteCodeStream,
  pub max_locals: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
  pub entries: Vec<Object>,
}

impl Pool {
  pub fn make() -> Self {
    Self { entries: vec![] }
  }

  pub fn append(&mut self, o: Object) -> usize {
    let index = self.entries.len();
    self.entries.push(o);
    index
  }

  pub fn get_by_index(&self, i: usize) -> Object {
    if self.entries.len() <= i {
      panic!("[Error]: Pool index out of range")
    }
    self.entries[i].clone()
  }
}
