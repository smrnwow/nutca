use super::{Formula, Tokenizer};
use crate::chemistry::table::Table;
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
        let mut formula_builder = Formula::new();

        let compound = Tokenizer::new(formula).tokenize()?;

        for element in compound.elements() {
            match self.table.element(element.symbol.as_str()) {
                Some(chemical_element) => {
                    formula_builder.add_element(chemical_element, element.subscript);
                }
                None => {
                    let message = format!("unknown element: {}", element.symbol);

                    return Err(Error::new(message));
                }
            }
        }

        formula_builder.calculate_molar_mass();

        formula_builder.calculate_mass_percent();

        Ok(formula_builder)
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;

    #[test]
    fn molar_mass() {
        let builder = Builder::new();

        assert_eq!(builder.build("KNO3").unwrap().molar_mass() as i32, 101);
    }
}
