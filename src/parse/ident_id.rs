#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct IdentId(pub(super) usize);

impl IdentId {
    pub(super) fn next(&mut self) -> Self {
        self.0 += 1;
        *self
    }
}
