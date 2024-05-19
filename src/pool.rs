use crate::object::Object;

pub struct Pool {
    entries: Vec<Object>,
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

    pub fn get_by_index(&mut self, i: usize) -> Object {
        if self.entries.len() <= i {
            panic!("[Error]: Pool out of range")
        }
        self.entries[i].clone()
    }
}
