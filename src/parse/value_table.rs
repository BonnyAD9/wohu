use std::collections::HashMap;

use crate::parse::IdentId;

#[derive(Debug)]
pub struct ValueTable<T> {
    tables: Vec<HashMap<IdentId, T>>,
}

impl<T> ValueTable<T> {
    pub fn new() -> Self {
        Self {
            tables: vec![HashMap::new()],
        }
    }

    pub fn set(&mut self, id: IdentId, value: T) {
        assert!(!self.tables.is_empty());
        self.tables.last_mut().unwrap().insert(id, value);
    }

    pub fn get(&self, id: IdentId) -> Option<&T> {
        for t in self.tables.iter().rev() {
            if let Some(v) = t.get(&id) {
                return Some(v);
            }
        }
        None
    }

    pub fn new_scope(&mut self) {
        self.tables.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.tables.pop();
        if self.tables.is_empty() {
            self.new_scope();
        }
    }
}

impl<T> Default for ValueTable<T> {
    fn default() -> Self {
        Self::new()
    }
}
