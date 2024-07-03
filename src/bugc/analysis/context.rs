use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use bug::stdlib::list_native_fns;
use bug::{FnPrototype, Type};

#[derive(Debug, Clone)]
pub enum MetaObject {
    VarType(Type),
    FnPrototype(FnPrototype),
}

#[derive(Debug, PartialEq)]
pub enum ContextType {
    Global,
    Function,
}

pub struct Context {
    pub type_: ContextType,
    store: HashMap<String, MetaObject>,
    parent: Option<Rc<RefCell<Context>>>,
}

impl Context {
    pub fn make_global() -> Self {
        let mut store: HashMap<String, MetaObject> = HashMap::new();
        for (name, native_fn) in list_native_fns() {
            store.insert(name, MetaObject::FnPrototype(native_fn.prototype));
        }
        Self {
            type_: ContextType::Global,
            store,
            parent: None,
        }
    }

    pub fn make(type_: ContextType, parent: Rc<RefCell<Context>>) -> Self {
        Self {
            type_,
            store: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn declare(&mut self, name: String, val: MetaObject) {
        self.store.insert(name, val);
    }

    pub fn is_declared(&self, name: &str) -> bool {
        if self.store.contains_key(name) {
            return true;
        }
        match self.parent {
            Some(ref parent) => parent.borrow().is_declared(name),
            None => false,
        }
    }

    pub fn lookup(&self, name: &str) -> Option<MetaObject> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone());
        }
        match self.parent {
            Some(ref parent) => parent.borrow().lookup(name),
            None => None,
        }
    }
}
