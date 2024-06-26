use std::collections::HashMap;

use super::Type;
use spider_vm::stdlib::list_native_fns;

pub struct MetaFunction {
    pub arity: u8,
    pub return_type: Type,
}

pub enum MetaObject {
    MetaFunction(MetaFunction),
}

pub struct MetaScope {
    pub store: HashMap<String, MetaObject>,
    pub typestack: Vec<Type>,
}

impl MetaScope {
    pub fn new() -> Self {
        let mut store: HashMap<String, MetaObject> = HashMap::new();
        let builtins = list_native_fns();
        for (fn_name, b) in builtins {
            store.insert(
                fn_name,
                MetaObject::MetaFunction(MetaFunction {
                    arity: b.prototype.arity,
                    return_type: b.prototype.return_type,
                }),
            );
        }

        Self {
            store,
            typestack: vec![],
        }
    }

    pub fn insert(&mut self, name: String, object: MetaObject) {
        self.store.insert(name, object);
    }

    pub fn exists_in_current(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }

    pub fn lookup_global(&self, name: &str) -> Option<&MetaObject> {
        self.store.get(name)
    }
}
