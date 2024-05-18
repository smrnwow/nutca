use super::nutrient::Nutrient;
use ellp::problem::VariableId;
use std::collections::HashMap;

pub struct Composition<'a> {
    nutrients: HashMap<&'a str, Nutrient<'a>>,
}

impl<'a> Composition<'a> {
    pub fn new(source: &'a Vec<Nutrient<'a>>) -> Self {
        let mut nutrients: HashMap<&'a str, Nutrient<'a>> = HashMap::new();

        source.iter().for_each(|nutrient| {
            nutrients.insert(nutrient.symbol(), nutrient.clone());
        });

        Self { nutrients }
    }

    pub fn add_coefficient(&mut self, symbol: &'a str, coefficient: (VariableId, f64)) {
        if let Some(nutrient) = self.nutrients.get_mut(symbol) {
            nutrient.add_coefficient(coefficient);
        }
    }

    pub fn nutrients(&self) -> Vec<&Nutrient> {
        self.nutrients.values().collect()
    }
}
