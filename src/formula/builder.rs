use super::{Formula, Tokenizer};
use crate::chemistry::Table;
use crate::error::Error;

pub struct Builder {
    table: Table,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    pub fn build(&self, formula: &str) -> Result<Formula, Error> {
        let compound = Tokenizer::new(&self.table, formula).tokenize()?;

        let formula_builder = Formula::from_compound(compound);

        Ok(formula_builder)
    }
}

#[cfg(test)]
mod tests {}
