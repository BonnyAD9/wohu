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

    let mut outputs: Vec<Box<dyn OutFmt>> = vec![];
    for (t, o) in args.outputs {
        let out: Box<dyn Write> = if let Some(f) = o {
            Box::new(BufWriter::new(File::create(f)?))
        } else {
            Box::new(stdout().lock())
        };

        let of: Box<dyn OutFmt> = match t {
            FmtType::Text => Box::new(out_fmt::Text::new(out)),
            FmtType::LatexSlides => Box::new(out_fmt::Latex::new(out)),
        };
        outputs.push(of);
    }

    for o in &mut outputs {
        o.init()?;
    }
    for (i, ip) in args.input.iter().enumerate() {
        let s = parse_file(ip)?;
        for o in &mut outputs {
            if i != 0 {
                o.song_space()?;
            }
            let cfg = s.configs.get(&s.default).unwrap();
            for (i, v) in cfg.verses.iter().enumerate() {
                if i != 0 {
                    o.verse_space()?;
                }
                o.write_verse(v.as_slice())?;
            }
        }
    }
    for o in &mut outputs {
        o.finalize()?;
    }

    Ok(())
}
