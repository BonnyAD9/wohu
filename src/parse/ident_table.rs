use std::collections::HashMap;

use crate::parse::{IdentId, ident::Ident};

#[derive(Debug)]
pub struct IdentTable {
    idents: HashMap<IdentId, Ident>,
    id: IdentId,
    tables: Vec<HashMap<String, Ident>>,
}

impl IdentTable {
    pub fn new() -> Self {
        Self {
            idents: HashMap::new(),
            id: IdentId(0),
            tables: vec![HashMap::new()],
        }
    }

    pub fn get_id(&mut self, n: &str) -> IdentId {
        assert!(!self.tables.is_empty());

        for t in self.tables.iter().rev() {
            if let Some(i) = t.get(n) {
                return i.id;
            }
        }

        let id = self.id.next();
        let ident = Ident::new(id, n.to_string());

        self.tables
            .last_mut()
            .unwrap()
            .insert(n.to_string(), ident.clone());
        self.idents.insert(id, ident);

        id
    }

    pub fn get_ident(&self, id: IdentId) -> Option<&Ident> {
        self.idents.get(&id)
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
