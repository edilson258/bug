use std::collections::HashMap;

use crate::{FunctionPrototype, Object, Type};

pub struct NativeFn {
  pub prototype: FunctionPrototype,
  pub function: fn(Vec<Object>) -> Option<Object>,
}

fn write_fn(args: Vec<Object>) -> Option<Object> {
  for object in args {
    println!("{object}");
  }
  None
}

pub fn list_natives() -> HashMap<String, NativeFn> {
  let write_fn_prototype = FunctionPrototype::new(1, Type::Void, vec![]);
  let write_fn = NativeFn { prototype: write_fn_prototype, function: write_fn };
  let mut fns: HashMap<String, NativeFn> = HashMap::new();
  fns.insert("write".to_string(), write_fn);
  fns
}
