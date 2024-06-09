use std::collections::HashMap;

use super::Type;

pub struct MetaFunction {
    pub arity: u8,
    pub return_type: Type,
}

pub enum MetaObject {
    MetaFunction(MetaFunction),
}

pub struct MetaScope {
    store: HashMap<String, MetaObject>,
}

impl MetaScope {
    pub fn new() -> Self {
        let write_fn = MetaFunction {
            arity: 1,
            return_type: Type::Void,
        };

        let mut store: HashMap<String, MetaObject> = HashMap::new();
        store.insert("write".to_string(), MetaObject::MetaFunction(write_fn));

        Self { store }
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
