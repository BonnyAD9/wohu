use std::{
    fs::File,
    io::{BufWriter, Write, stdout},
    process::ExitCode,
};

use anyhow::Result;
use pareg::Pareg;

use crate::{
    cli::Args,
    out_fmt::{FmtType, OutFmt},
    parse::parse_file,
};

mod cli;
mod data;
mod out_fmt;
mod parse;

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
        FmtType::Text => process_inputs(&args.input, out_fmt::Text::new(out)),
        FmtType::Latex => {
            process_inputs(&args.input, out_fmt::Latex::new(out))
        }
    }
}

fn process_inputs<O: OutFmt>(inputs: &[String], mut o: O) -> Result<()> {
    o.init()?;
    for (i, ip) in inputs.iter().enumerate() {
        if i != 0 {
            o.song_space()?;
        }
        let s = parse_file(ip)?;
        let cfg = s.configs.get(&s.default).unwrap();
        for (i, v) in cfg.verses.iter().enumerate() {
            if i != 0 {
                o.verse_space()?;
            }
            o.write_verse(v.as_slice())?;
        }
    }
    o.finalize()?;
    Ok(())
}
