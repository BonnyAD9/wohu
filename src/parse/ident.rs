use crate::parse::IdentId;

#[derive(Debug, Clone)]
pub struct Ident {
    pub id: IdentId,
    pub name: String,
}

impl Ident {
    pub(super) fn new(id: IdentId, name: String) -> Self {
        Self { id, name }
    }
}
