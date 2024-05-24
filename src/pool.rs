use crate::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionRef {
    pub name_index: usize,
    pub signature_index: usize,
    pub fn_index: usize, // index within the list of functions
}

impl FunctionRef {
    pub fn make(name_index: usize, signature_index: usize, fn_index: usize) -> Self {
        Self {
            name_index,
            signature_index,
            fn_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PoolEntry {
    FunctionRef(FunctionRef),
    Object(Object),
    Utf8(String),
}

type PoolEntries = Vec<PoolEntry>;

pub struct Pool {
    pub entries: PoolEntries,
}

impl Pool {
    pub fn make() -> Self {
        Self { entries: vec![] }
    }

    pub fn append(&mut self, pentry: PoolEntry) -> usize {
        let index = self.entries.len();
        self.entries.push(pentry);
        index
    }

    pub fn get_by_index(&self, i: usize) -> PoolEntry {
        if self.entries.len() <= i {
            panic!("[Error]: Pool index out of range")
        }
        self.entries[i].clone()
    }
}
