use super::formula::Formula;
use super::tokenization::tokenizer::Tokenizer;
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

    pub fn build(&self, formula: &str) -> Result<Formula, Vec<Error>> {
        let mut errors: Vec<Error> = Vec::new();

        let mut formula_builder = Formula::new();

        let compound = Tokenizer::new(formula).tokenize();

        compound.elements().iter().for_each(|element| {
            match self.table.element(element.symbol.as_str()) {
                Some(chemical_element) => {
                    formula_builder.add_element(chemical_element, element.subscript);
                }
                None => {
                    let message = format!("UnexpectedElement: {}", element.symbol);

                    errors.push(Error::new(message));
                }
            }
        });

        if errors.len() > 0 {
            return Err(errors);
        }

        if compound.coefficient() > 1 {
            formula_builder.multiple(compound.coefficient());
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
