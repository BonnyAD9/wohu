use std::sync::Arc;

use crate::parse::IdentId;

#[derive(Debug)]
pub struct SongConf {
    pub language: IdentId,
    pub _name: String,
    pub verses: Vec<Arc<Vec<String>>>,
}
