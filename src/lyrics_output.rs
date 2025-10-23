use anyhow::Result;

pub trait LyricsOutput {
    fn init(&mut self) -> Result<()>;
    fn write_verse(&mut self, verse: &[String]) -> Result<()>;
    fn song_space(&mut self) -> Result<()>;
    fn finalize(&mut self) -> Result<()>;
}
