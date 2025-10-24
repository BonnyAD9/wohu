use anyhow::Result;

mod fmt_type;
mod latex;
mod text;

pub use self::{fmt_type::*, latex::*, text::*};

pub trait OutFmt {
    fn init(&mut self) -> Result<()>;
    fn write_verse(&mut self, verse: &[String]) -> Result<()>;
    fn song_space(&mut self) -> Result<()>;
    fn verse_space(&mut self) -> Result<()>;
    fn finalize(&mut self) -> Result<()>;
}
