use super::Symbol;

#[derive(Debug)]
pub struct Table {
    elements: Vec<Symbol>,
}

impl Table {
    pub fn new() -> Table {
        let table = Table {
            elements: vec![
                Symbol::Nitrogen,
                Symbol::Phosphorus,
                Symbol::Potassium,
                Symbol::Calcium,
                Symbol::Magnesium,
                Symbol::Sulfur,
                Symbol::Iron,
                Symbol::Zink,
                Symbol::Manganese,
                Symbol::Boron,
                Symbol::Copper,
                Symbol::Molybdenum,
                Symbol::Hydrogen,
                Symbol::Carbon,
                Symbol::Oxygen,
                Symbol::Sodium,
                Symbol::Aluminium,
                Symbol::Silicon,
                Symbol::Chlorine,
                Symbol::Cobalt,
            ],
        };

        table
    }

    pub fn by_symbol(&self, symbol: &str) -> Option<Symbol> {
        let element = self
            .elements
            .iter()
            .find(|element| element.symbol() == symbol);

        match element {
            Some(element) => Some(element.clone()),
            None => None,
        }
    }

    pub fn nutrients(&self) -> Vec<&Symbol> {
        self.elements
            .iter()
            .filter(|element| element.nutrient().is_some())
            .collect()
    }
}
