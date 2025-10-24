use std::io::Write;

use crate::out_fmt::OutFmt;

pub struct Latex<W: Write> {
    writer: W,
}

impl<W: Write> Latex<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> OutFmt for Latex<W> {
    fn init(&mut self) -> anyhow::Result<()> {
        writeln!(
            self.writer,
            "\\documentclass[17pt]{{beamer}}
\\usepackage{{helvet}}
\\usepackage[czech]{{babel}}

\\setbeamercolor{{background canvas}}{{bg=black}}
\\setbeamercolor{{normal text}}{{fg=white}}
\\setbeamertemplate{{navigation symbols}}{{}}
\\setbeamertemplate{{footline}}{{\\textcolor{{darkgray}}\
    {{\\insertframenumber}}}}

\\begin{{document}}

\\begin{{frame}}
\\end{{frame}}
"
        )?;
        Ok(())
    }

    fn write_verse(&mut self, verse: &[String]) -> anyhow::Result<()> {
        writeln!(
            self.writer,
            "\\begin{{frame}}
\\begin{{center}}
\\textbf{{"
        )?;
        for l in verse {
            writeln!(self.writer, "    {l}")?;
        }
        writeln!(
            self.writer,
            "}}
\\end{{center}}
\\end{{frame}}
"
        )?;
        Ok(())
    }

    fn song_space(&mut self) -> anyhow::Result<()> {
        writeln!(
            self.writer,
            "\\begin{{frame}}
\\end{{frame}}
"
        )?;
        Ok(())
    }

    fn verse_space(&mut self) -> anyhow::Result<()> {
        writeln!(self.writer)?;
        Ok(())
    }

    fn finalize(&mut self) -> anyhow::Result<()> {
        writeln!(
            self.writer,
            "\\begin{{frame}}
\\end{{frame}}

\\end{{document}}"
        )?;
        Ok(())
    }
}
