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
pub struct FunctionPrototype {
  pub arity: usize,
  pub return_type: Type,
  pub parameters_types: Vec<Type>,
}

impl FunctionPrototype {
  pub fn new(arity: usize, return_type: Type, parameters_types: Vec<Type>) -> Self {
    Self { arity, return_type, parameters_types }
  }
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
  pub start_line: usize,
  pub arity: usize,
  pub code: ByteCodeStream,
  pub max_locals: usize,
}

impl DefinedFn {
  pub fn new(start_line: usize, arity: usize, code: ByteCodeStream, max_locals: usize) -> Self {
    Self { start_line, arity, code, max_locals }
  }
}

impl Default for DefinedFn {
  fn default() -> Self {
    Self { start_line: 0, arity: 0, code: ByteCodeStream::empty(), max_locals: 0 }
  }
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

  pub fn get_by_index(&self, i: usize) -> Option<&Object> {
    if self.entries.len() <= i {
      return None;
    }
    Some(&self.entries[i])
  }
}
