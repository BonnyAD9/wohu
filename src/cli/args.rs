use std::path::PathBuf;

use anyhow::Result;
use pareg::Pareg;

use crate::out_fmt::OutFmt;

#[derive(Debug, Default)]
pub struct Args {
    pub input: Vec<String>,
    pub fmt: OutFmt,
    pub output: Option<PathBuf>,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Self::default();

        while let Some(a) = args.next() {
            match a {
                "-o" | "--output" => res.output = Some(args.next_arg()?),
                "-i" | "--input" => res.input.push(args.next_arg()?),
                "-f" | "--fmt" | "--format" => res.fmt = args.next_arg()?,
                "--text" => res.fmt = OutFmt::Text,
                v if v.starts_with('-') => {
                    return Err(args.err_unknown_argument().hint("Use `-i` to specify input file that starts with `-`.").into());
                }
                v => res.input.push(v.to_string()),
            }
        }

        Ok(res)
    }
}
