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
    pub name: String,
    pub prototype: BuiltinFnPrototype,
    pub function: fn(Object),
}

fn write_fn(object: Object) {
    match object {
        Object::Int(x) => println!("{}", x),
        _ => todo!(),
    };
}

pub fn list_builtin_fns() -> Vec<BuiltinFn> {
    let write_fn_prototype = BuiltinFnPrototype {
        arity: 1,
        argtypes: vec![],
        return_type: Type::Void,
    };

    let write_fn = BuiltinFn {
        name: "write".to_string(),
        prototype: write_fn_prototype,
        function: write_fn,
    };

    vec![write_fn]
}
