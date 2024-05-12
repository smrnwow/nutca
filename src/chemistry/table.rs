use super::element::Element;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Table {
    elements: HashMap<&'static str, Element>,
}

impl Table {
    pub fn new() -> Table {
        let table = Table {
            elements: HashMap::from([
                ("H", Element::new("H", 1.00794)),
                ("B", Element::new("B", 10.811).nutrient()),
                ("C", Element::new("C", 12.0107)),
                ("N", Element::new("N", 14.0067).nutrient()),
                ("O", Element::new("O", 15.9994)),
                ("Na", Element::new("Na", 22.98976928)),
                ("Mg", Element::new("Mg", 24.3050).nutrient()),
                ("Al", Element::new("Al", 26.9815386)),
                ("Si", Element::new("Si", 28.0855)),
                ("P", Element::new("P", 30.973762).nutrient()),
                ("S", Element::new("S", 32.065).nutrient()),
                ("Cl", Element::new("Cl", 35.453)),
                ("K", Element::new("K", 39.0983).nutrient()),
                ("Ca", Element::new("Ca", 40.078).nutrient()),
                ("Mn", Element::new("Mn", 54.938045).nutrient()),
                ("Fe", Element::new("Fe", 55.845).nutrient()),
                ("Co", Element::new("Co", 58.933195)),
                ("Cu", Element::new("Cu", 63.546).nutrient()),
                ("Zn", Element::new("Zn", 65.409).nutrient()),
                ("Mo", Element::new("Mo", 95.94).nutrient()),
                ("I", Element::new("I", 126.90)),
                ("Cr", Element::new("Cr", 51.99)),
                ("Ni", Element::new("Ni", 58.70)),
                ("Se", Element::new("Se", 34.)),
                ("Br", Element::new("Br", 35.)),
            ]),
        };

        table
    }

    pub fn element(&self, symbol: &str) -> Option<Element> {
        match self.elements.get(symbol) {
            Some(element) => Some(element.clone()),
            None => None,
        }
    }

    pub fn nutrients(&self) -> Vec<&str> {
        self.elements
            .values()
            .filter(|element| element.is_nutrient())
            .map(|element| element.symbol)
            .collect()
    }
}
