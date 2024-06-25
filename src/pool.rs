use serde::{Deserialize, Serialize};

use crate::object::Object;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PoolEntry {
    Object(Object),
}

type PoolEntries = Vec<PoolEntry>;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
