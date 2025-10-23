use std::collections::HashMap;

use crate::{data::SongConf, parse::IdentId};

#[derive(Debug)]
pub struct Song {
    pub configs: HashMap<IdentId, SongConf>,
    pub default: IdentId,
}
