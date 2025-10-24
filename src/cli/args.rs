use std::path::PathBuf;

use anyhow::Result;
use pareg::Pareg;

use crate::out_fmt::FmtType;

#[derive(Debug, Default)]
pub struct Args {
    pub input: Vec<String>,
    pub outputs: Vec<(FmtType, Option<PathBuf>)>,
}

impl Args {
    pub fn parse(mut args: Pareg) -> Result<Self> {
        let mut res = Self::default();
        let mut output = None;

        while let Some(a) = args.next() {
            match a {
                "-o" | "--output" => output = Some(args.next_arg()?),
                "--stdout" => output = None,
                "-i" | "--input" => res.input.push(args.next_arg()?),
                "-f" | "--fmt" | "--format" => {
                    res.outputs.push((args.next_arg()?, output.clone()))
                }
                "--text" => res.outputs.push((FmtType::Text, output.clone())),
                "--tex-slides" | "--latex-slides" => {
                    res.outputs.push((FmtType::LatexSlides, output.clone()))
                }
                v if v.starts_with('-') => {
                    return Err(args
                        .err_unknown_argument()
                        .hint(
                            "Use `-i` to specify input file that starts \
                                with `-`.",
                        )
                        .into());
                }
                v => res.input.push(v.to_string()),
            }
        }

        Ok(res)
    }
}
