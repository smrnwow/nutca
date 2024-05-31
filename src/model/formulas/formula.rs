use super::{Element, NitrogenForms, Tokenizer};
use crate::model::chemistry::{NitrogenForm, NutrientAmount, Symbol, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Formula {
    formulation: String,
    elements: HashMap<Symbol, i32>,
    nutrients: [NutrientAmount; 12],
    nitrogen_forms: NitrogenForms,
}

impl Formula {
    pub fn new(formulation: &str) -> Self {
        Self {
            formulation: formulation.to_string(),
            elements: HashMap::new(),
            nutrients: [
                NutrientAmount::Nitrogen(0.0),
                NutrientAmount::Phosphorus(0.0),
                NutrientAmount::Potassium(0.0),
                NutrientAmount::Calcium(0.0),
                NutrientAmount::Magnesium(0.0),
                NutrientAmount::Sulfur(0.0),
                NutrientAmount::Iron(0.0),
                NutrientAmount::Zinc(0.0),
                NutrientAmount::Manganese(0.0),
                NutrientAmount::Boron(0.0),
                NutrientAmount::Copper(0.0),
                NutrientAmount::Molybdenum(0.0),
            ],
            nitrogen_forms: NitrogenForms::new(),
        }
    }

    pub fn nutrients(&self) -> Vec<NutrientAmount> {
        Vec::from(self.nutrients)
    }

    pub fn nitrogen_forms(&self) -> Vec<NitrogenForm> {
        let nitrogen_percent = self.nutrients[NutrientAmount::Nitrogen(0.0).index()].value();

        self.nitrogen_forms.values(nitrogen_percent)
    }

    pub fn formulation(&self) -> String {
        self.formulation.clone()
    }

    fn add_element(&mut self, element: &Element) {
        match self.elements.get_mut(&element.symbol()) {
            Some(elem) => {
                *elem += element.subscript();
            }
            None => {
                self.elements.insert(element.symbol(), element.subscript());
            }
        }

        self.nitrogen_forms.find(element.clone());
    }

    fn apply_coefficient(&mut self, coefficient: i32) {
        if coefficient > 1 {
            self.elements.values_mut().for_each(|atoms_count| {
                *atoms_count *= coefficient;
            });
        }
    }

    fn molar_mass(&self) -> f64 {
        self.elements
            .iter()
            .map(|(element, atoms_count)| element.atomic_weight() * *atoms_count as f64)
            .sum()
    }

    fn setup_nutrients(&mut self) {
        let molar_mass = self.molar_mass();

        self.elements.iter().for_each(|(element, atoms_count)| {
            let atoms_count = *atoms_count as f64;

            let percent = (element.atomic_weight() * atoms_count) / molar_mass * 100.;

            if let Some(nutrient_percent) = match element {
                Symbol::Nitrogen => Some(NutrientAmount::Nitrogen(percent)),
                Symbol::Phosphorus => Some(NutrientAmount::Phosphorus(percent)),
                Symbol::Potassium => Some(NutrientAmount::Potassium(percent)),
                Symbol::Calcium => Some(NutrientAmount::Calcium(percent)),
                Symbol::Magnesium => Some(NutrientAmount::Magnesium(percent)),
                Symbol::Sulfur => Some(NutrientAmount::Sulfur(percent)),
                Symbol::Iron => Some(NutrientAmount::Iron(percent)),
                Symbol::Zink => Some(NutrientAmount::Zinc(percent)),
                Symbol::Manganese => Some(NutrientAmount::Manganese(percent)),
                Symbol::Boron => Some(NutrientAmount::Boron(percent)),
                Symbol::Copper => Some(NutrientAmount::Copper(percent)),
                Symbol::Molybdenum => Some(NutrientAmount::Molybdenum(percent)),
                _ => None,
            } {
                self.nutrients[nutrient_percent.index()] = nutrient_percent;
            }
        });
    }
}

impl From<String> for Formula {
    fn from(formulation: String) -> Self {
        Self::from(formulation.as_str())
    }
}

impl From<&str> for Formula {
    fn from(formulation: &str) -> Self {
        let mut formula = Self::new(formulation);

        let compound = Tokenizer::new(&Table::new(), formulation)
            .tokenize()
            .unwrap();

        compound.composition().iter().for_each(|element| {
            formula.add_element(element);
        });

        compound.hydrate().iter().for_each(|element| {
            formula.add_element(element);
        });

        formula.apply_coefficient(compound.coefficient());

        formula.setup_nutrients();

        formula
    }
}

#[cfg(test)]
mod tests {
    use super::Formula;

    #[test]
    fn nitrogen_form() {
        // "KNO3"
        // "NH4NO3"
        // "Ca(NO3)2"
        // "2C14H18N3O10Fe(NH4)2"

        let formula = Formula::from("NH4NO3");

        println!("{:#?}", formula);

        println!("{:#?}", formula.nitrogen_forms());
    }
}
