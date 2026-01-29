use crate::parse::IdentId;

#[derive(Debug)]
pub struct SongConf {
    pub language: IdentId,
    pub _name: String,
    pub verses: Vec<Vec<String>>,
}
