use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Integer,
    String,
    Void,
}

#[derive(Debug, Clone)]
pub struct BuiltinFnPrototype {
    pub arity: u8,
    pub argtypes: Vec<Type>,
    pub return_type: Type,
}

pub struct BuiltinFn {
    pub prototype: BuiltinFnPrototype,
    pub function: fn(Vec<Object>),
}

fn write_fn(args: Vec<Object>) {
    for object in args {
        match object {
            Object::Int(val) => println!("{val}"),
            Object::String(val) => println!("{val}"),
            Object::Null => println!("null"),
        }
    }
}

pub fn list_builtin_fns() -> HashMap<String, BuiltinFn> {
    let write_fn_prototype = BuiltinFnPrototype {
        arity: 1,
        argtypes: vec![],
        return_type: Type::Void,
    };
    let write_fn = BuiltinFn {
        prototype: write_fn_prototype,
        function: write_fn,
    };
    let mut fns: HashMap<String, BuiltinFn> = HashMap::new();
    fns.insert("write".to_string(), write_fn);
    fns
}
