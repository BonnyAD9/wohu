use std::io::Write;

use crate::lyrics_output::LyricsOutput;

pub struct TextOutput<W: Write> {
    writer: W,
}

impl<W: Write> TextOutput<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> LyricsOutput for TextOutput<W> {
    fn init(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn write_verse(&mut self, verse: &[String]) -> anyhow::Result<()> {
        for l in verse {
            writeln!(self.writer, "{l}")?;
        }
        Ok(())
    }

    fn song_space(&mut self) -> anyhow::Result<()> {
        writeln!(self.writer)?;
        Ok(())
    }

    fn finalize(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
