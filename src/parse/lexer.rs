use std::mem;

use anyhow::{Result, anyhow, bail};

use crate::parse::{IdentId, ident_table::IdentTable, token::Token};

#[derive(Debug)]
pub struct Lexer<I: Iterator<Item = Result<char>>> {
    it: I,
    cur: Option<char>,
    pub idents: IdentTable,
    id: IdentId,
    buf: String,
    lines: Option<Vec<String>>,
}

impl<I: Iterator<Item = Result<char>>> Lexer<I> {
    pub fn new(mut it: I) -> Result<Self> {
        let cur = it.next().transpose()?;
        Ok(Self {
            it,
            cur,
            idents: IdentTable::new(),
            id: IdentId(0),
            buf: String::new(),
            lines: None,
        })
    }

    pub fn next(&mut self) -> Result<Token> {
        while matches!(self.cur, Some(c) if c.is_ascii_whitespace()) {
            self.next_chr()?;
        }

        match self.cur {
            Some(c) if c.is_ascii_alphanumeric() => self.next_alnum(),
            Some('"') => self.next_string(),
            Some('#') => self.next_declare(),
            None => Ok(Token::Eof),
            _ => self.next_op_punct(),
        }
    }

    pub fn last_string(&self) -> &str {
        &self.buf
    }

    pub fn last_id(&self) -> IdentId {
        self.id
    }

    pub fn last_line_string(&mut self) -> Vec<String> {
        assert!(self.lines.is_some());
        mem::take(&mut self.lines).unwrap()
    }

    fn next_alnum(&mut self) -> Result<Token> {
        self.buf.clear();
        self.read_alnum()?;

        match self.buf.as_str() {
            "name" => return Ok(Token::KwName),
            "order" => return Ok(Token::KwOrder),
            _ => {}
        }

        if self.cur != Some('{') {
            self.id = self.idents.get_id(&self.buf);
            return Ok(Token::Ident);
        }

        match self.buf.as_str() {
            "l" => self.next_line_string(),
            s => bail!("Unknonw special string type `{s}`."),
        }
    }

    fn next_declare(&mut self) -> Result<Token> {
        self.next_chr()?; // #
        self.buf.clear();
        self.read_alnum()?;

        match self.buf.as_str() {
            "language" => Ok(Token::DecLanguage),
            s => bail!("Unknown declaration `{s}`"),
        }
    }

    fn next_op_punct(&mut self) -> Result<Token> {
        assert!(self.cur.is_some());

        let c = self.cur.unwrap();
        self.next_chr()?;

        let tok = match c {
            ':' => Token::Colon,
            '=' => Token::Assign,
            '+' => Token::Add,
            ',' => Token::Comma,
            '{' => Token::OpenBracket,
            '}' => Token::CloseBracket,
            '[' => Token::OpenSq,
            ']' => Token::CloseSq,
            c => bail!("Unknown operator or punctuation `{c}`."),
        };

        Ok(tok)
    }

    fn next_string(&mut self) -> Result<Token> {
        self.buf.clear();

        loop {
            self.next_chr()?;
            match self.cur {
                Some('"') => break,
                Some('\\') => {
                    self.next_chr()?;
                    self.buf
                        .push(self.cur.ok_or_else(|| {
                            anyhow!("Missing closing '\"'.")
                        })?);
                }
                Some(c) => self.buf.push(c),
                None => bail!("Missing closing '\"'."),
            }
        }

        self.next_chr()?; // "
        Ok(Token::String)
    }

    fn next_line_string(&mut self) -> Result<Token> {
        let mut lines = vec![];
        let mut line = String::new();

        loop {
            self.next_chr()?;
            if self.cur == Some('\n') {
                if !line.is_empty() {
                    lines.push(mem::take(&mut line));
                }
                continue;
            }
            let space = matches!(self.cur, Some(c) if c.is_ascii_whitespace());
            while matches!(self.cur, Some(c) if c.is_ascii_whitespace()) {
                self.next_chr()?;
            }
            if !line.is_empty() && space {
                line.push(' ');
            }

            match self.cur {
                Some('}') => break,
                Some('\\') => {
                    self.next_chr()?;
                    line.push(
                        self.cur
                            .ok_or_else(|| anyhow!("Missing closing '}}'."))?,
                    );
                }
                Some(c) => line.push(c),
                None => bail!("Missing closing '}}'."),
            }
        }

        self.next_chr()?; // }

        if !line.is_empty() {
            lines.push(line);
        }

        self.lines = Some(lines);

        Ok(Token::LineString)
    }

    fn read_alnum(&mut self) -> Result<()> {
        while let Some(c) = self.cur {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            self.buf.push(c);
            self.next_chr()?;
        }
        Ok(())
    }

    fn next_chr(&mut self) -> Result<Option<char>> {
        self.cur = self.it.next().transpose()?;
        Ok(self.cur)
    }
}
