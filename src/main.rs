use std::{
    fs::File,
    io::{BufWriter, Write, stdout},
    process::ExitCode,
};

use anyhow::Result;
use pareg::Pareg;

use crate::{
    cli::Args, lyrics_output::LyricsOutput, out_fmt::OutFmt,
    parse::parse_file, text_output::TextOutput,
};

mod cli;
mod data;
mod lyrics_output;
mod out_fmt;
mod parse;
mod text_output;

fn main() -> ExitCode {
    match start() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn start() -> Result<()> {
    let args = Args::parse(Pareg::args())?;

    let out: Box<dyn Write> = if let Some(f) = args.output {
        Box::new(BufWriter::new(File::create(f)?))
    } else {
        Box::new(stdout().lock())
    };

    match args.fmt {
        OutFmt::Text => process_inputs(&args.input, TextOutput::new(out)),
    }
}

fn process_inputs<O: LyricsOutput>(inputs: &[String], mut o: O) -> Result<()> {
    o.init()?;
    for i in inputs {
        let s = parse_file(i)?;
        let cfg = s.configs.get(&s.default).unwrap();
        for (i, v) in cfg.verses.iter().enumerate() {
            if i != 0 {
                o.song_space()?;
            }
            o.write_verse(v.as_slice())?;
        }
    }
    o.finalize()?;
    Ok(())
}
