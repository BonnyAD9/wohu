use std::io::Write;

use crate::out_fmt::OutFmt;

pub struct Text<W: Write> {
    writer: W,
}

impl<W: Write> Text<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> OutFmt for Text<W> {
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
        writeln!(self.writer, "\n")?;
        Ok(())
    }

    fn verse_space(&mut self) -> anyhow::Result<()> {
        writeln!(self.writer)?;
        Ok(())
    }

    fn finalize(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
