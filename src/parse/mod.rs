mod expr;
mod ident;
mod ident_id;
mod ident_table;
mod lexer;
mod parser;
mod token;
mod value_table;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

use utf8_chars::BufReadCharsExt;

use crate::{
    data::Song,
    parse::{lexer::Lexer, parser::Parser},
};

pub use self::ident_id::*;

pub fn parse_iterator(
    i: impl IntoIterator<Item = Result<char>>,
) -> Result<Song> {
    let mut parser = Parser::new(Lexer::new(i.into_iter())?)?;
    parser.parse_song()
}

pub fn parse_read(mut r: impl BufRead) -> Result<Song> {
    parse_iterator(r.chars().map(|a| a.map_err(|e| e.into())))
}

pub fn parse_file(f: impl AsRef<Path>) -> Result<Song> {
    parse_read(BufReader::new(File::open(f)?))
}
