use anyhow::{Result, bail};

use crate::parse::{
    IdentId, ident_table::IdentTable, value_table::ValueTable,
};

#[derive(Debug)]
pub enum Expr {
    Value(Vec<String>),
    Ident(IdentId),
    Add(Vec<Expr>),
}

impl Expr {
    pub fn eval(
        &self,
        vt: &ValueTable<Expr>,
        idt: &IdentTable,
    ) -> Result<Vec<String>> {
        let mut stack = vec![(self, false)];
        let mut pred = vec![];
        let mut res = vec![];

        while let Some((expr, pop_pred)) = stack.pop() {
            match &expr {
                Expr::Value(v) => res.extend(v.iter().cloned()),
                Expr::Ident(i) => {
                    if pred.contains(i) {
                        bail!(
                            "Cycle detected: `{}` depends on itself.",
                            idt.get_name(*i)
                        );
                    }
                    pred.push(*i);
                    stack.push((vt.try_get(*i, idt)?, true));
                }
                Expr::Add(av) if !av.is_empty() => {
                    stack.push((&av[av.len() - 1], true));
                    stack.extend(av.iter().rev().skip(1).map(|a| (a, false)));
                }
                _ => {}
            }

            if pop_pred {
                pred.pop();
            }
        }

        Ok(res)
    }
}
