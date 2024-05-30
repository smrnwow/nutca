use super::{Compound, Element};
use crate::model::chemistry::{Nutrient, Symbol};
use crate::model::fertilizers::NutrientContent;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Formula {
    elements: HashMap<Symbol, i32>,
}

impl Formula {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn from_compound(compound: Compound) -> Self {
        let mut formula = Self::new();

        compound.composition().iter().for_each(|element| {
            formula.add_element(element);
        });

        compound.hydrate().iter().for_each(|element| {
            formula.add_element(element);
        });

        formula.apply_coefficient(compound.coefficient());

        formula
    }

    pub fn nutrients(&self) -> Vec<NutrientContent> {
        let mut nutrients: Vec<NutrientContent> = Vec::new();

        self.elements.iter().for_each(|(element, atoms_count)| {
            let atoms_count = *atoms_count as f64;

            let percent = (element.atomic_weight() * atoms_count) / self.molar_mass() * 100.;

            if let Some(nutrient) = element.nutrient() {
                if let Some(nutrient_percent) = match nutrient {
                    Nutrient::Nitrogen => Some(NutrientContent::Nitrogen(percent)),
                    Nutrient::Phosphorus => Some(NutrientContent::Phosphor(percent)),
                    Nutrient::Potassium => Some(NutrientContent::Potassium(percent)),
                    Nutrient::Calcium => Some(NutrientContent::Calcium(percent)),
                    Nutrient::Magnesium => Some(NutrientContent::Magnesium(percent)),
                    Nutrient::Sulfur => Some(NutrientContent::Sulfur(percent)),
                    Nutrient::Iron => Some(NutrientContent::Iron(percent)),
                    Nutrient::Zink => Some(NutrientContent::Zinc(percent)),
                    Nutrient::Manganese => Some(NutrientContent::Manganese(percent)),
                    Nutrient::Boron => Some(NutrientContent::Boron(percent)),
                    Nutrient::Copper => Some(NutrientContent::Copper(percent)),
                    Nutrient::Molybdenum => Some(NutrientContent::Molybdenum(percent)),
                    _ => None,
                } {
                    nutrients.push(nutrient_percent);
                }
            }
        });

        nutrients
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
}
