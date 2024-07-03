use std::collections::HashMap;

use crate::{FnPrototype, Object, Type};

pub struct NativeFn {
    pub prototype: FnPrototype,
    pub function: fn(Vec<Object>) -> Option<Object>,
}

fn write_fn(args: Vec<Object>) -> Option<Object> {
    for object in args {
        println!("{object}");
    }
    None
}

pub fn list_native_fns() -> HashMap<String, NativeFn> {
    let write_fn_prototype = FnPrototype {
        arity: 1,
        argtypes: vec![],
        return_type: Type::Void,
    };
    let write_fn = NativeFn {
        prototype: write_fn_prototype,
        function: write_fn,
    };
    let mut fns: HashMap<String, NativeFn> = HashMap::new();
    fns.insert("write".to_string(), write_fn);
    fns
}
