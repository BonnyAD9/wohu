use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, bail};

use crate::{
    data::{Song, SongConf},
    parse::{IdentId, lexer::Lexer, token::Token, value_table::ValueTable},
};

#[derive(Debug)]
pub struct Parser<I: Iterator<Item = Result<char>>> {
    lex: Lexer<I>,
    cur: Token,
    values: ValueTable<Arc<Vec<String>>>,
    names: HashMap<IdentId, String>,
    order: Vec<IdentId>,
}

impl<I: Iterator<Item = Result<char>>> Parser<I> {
    pub fn new(mut lex: Lexer<I>) -> Result<Self> {
        let cur = lex.next()?;
        Ok(Self {
            lex,
            cur,
            values: ValueTable::new(),
            names: HashMap::new(),
            order: vec![],
        })
    }

    pub fn parse_song(&mut self) -> Result<Song> {
        let mut configs = HashMap::new();
        let mut default = None;

        loop {
            match self.cur {
                Token::KwName => {
                    let names = self.parse_names()?;
                    self.names.extend(names);
                }
                Token::KwOrder => self.order = self.parse_order()?,
                Token::DecLanguage => {
                    let conf = self.parse_song_conf()?;
                    if default.is_none() {
                        default = Some(conf.language);
                    }
                    configs.insert(conf.language, conf);
                }
                Token::Ident => self.parse_verse()?,
                Token::Eof => break,
                t => bail!("Unexpected token `{t:?}`."),
            }
        }

        let default = if let Some(d) = default {
            d
        } else {
            let id = self.lex.idents.get_id("generic");
            let conf = self.construct_config(String::new(), None, id)?;
            configs.insert(id, conf);
            id
        };

        Ok(Song { default, configs })
    }

    fn parse_names(&mut self) -> Result<HashMap<IdentId, String>> {
        self.expect_nexts([Token::Assign, Token::OpenBracket])?;

        let mut res = HashMap::new();

        loop {
            self.next()?;
            if self.cur == Token::CloseBracket {
                break;
            }

            self.expect(Token::Ident)?;
            let id = self.lex.last_id();
            self.expect_nexts([Token::Colon, Token::String])?;
            let name = self.lex.last_string().to_owned();
            res.insert(id, name);

            self.skip_if(Token::Comma)?;
        }

        self.next()?; // }

        Ok(res)
    }

    fn parse_order(&mut self) -> Result<Vec<IdentId>> {
        self.expect_nexts([Token::Assign, Token::OpenSq])?;

        let mut res = vec![];

        loop {
            self.next()?;
            if self.cur == Token::CloseSq {
                break;
            }

            self.expect(Token::Ident)?;
            res.push(self.lex.last_id());

            self.skip_if(Token::Comma)?;
        }

        self.next()?; // ]

        Ok(res)
    }

    fn parse_song_conf(&mut self) -> Result<SongConf> {
        self.expect_next(Token::Ident)?;
        let language = self.lex.last_id();
        self.next()?; // language

        let mut name = None;
        let mut order = None;

        self.new_scope();
        loop {
            match self.cur {
                Token::KwName => name = Some(self.parse_name()?),
                Token::KwOrder => order = Some(self.parse_order()?),
                Token::Ident => self.parse_verse()?,
                Token::DecLanguage => break,
                Token::Eof => break,
                t => bail!("Unexpected token `{t:?}`."),
            }
        }

        let res =
            self.construct_config(name.unwrap_or_default(), order, language)?;
        self.pop_scope();

        Ok(res)
    }

    fn parse_verse(&mut self) -> Result<()> {
        let id = self.lex.last_id();
        self.expect_nexts([Token::Colon, Token::LineString])?;
        let text = self.lex.last_line_string();
        self.next()?;

        self.values.set(id, text.into());
        Ok(())
    }

    fn parse_name(&mut self) -> Result<String> {
        self.expect_nexts([Token::Assign, Token::String])?;
        let name = self.lex.last_string().to_owned();
        self.next()?;
        Ok(name)
    }

    fn construct_config(
        &self,
        name: String,
        order: Option<Vec<IdentId>>,
        language: IdentId,
    ) -> Result<SongConf> {
        let mut verses = vec![];
        for id in order.as_ref().unwrap_or(&self.order) {
            let Some(v) = self.values.get(*id) else {
                let name = &self.lex.idents.get_ident(*id).unwrap().name;
                bail!("Missing verse text for the identifier `{name}`.");
            };
            verses.push(v.clone())
        }
        Ok(SongConf {
            language,
            _name: name,
            verses,
        })
    }

    fn new_scope(&mut self) {
        self.values.new_scope();
        self.lex.idents.new_scope();
    }

    fn pop_scope(&mut self) {
        self.values.pop_scope();
        self.lex.idents.pop_scope();
    }

    fn skip_if(&mut self, i: Token) -> Result<()> {
        if self.cur == i {
            self.next()?;
        }
        Ok(())
    }

    fn expect_nexts(
        &mut self,
        i: impl IntoIterator<Item = Token>,
    ) -> Result<()> {
        for t in i {
            self.expect_next(t)?;
        }
        Ok(())
    }

    fn expect_next(&mut self, t: Token) -> Result<()> {
        self.next()?;
        self.expect(t)
    }

    fn expect(&mut self, t: Token) -> Result<()> {
        if self.cur != t {
            bail!("Expected the token `{t:?}` but found `{:?}`", self.cur);
        }
        Ok(())
    }

    fn next(&mut self) -> Result<Token> {
        self.cur = self.lex.next()?;
        Ok(self.cur)
    }
}
