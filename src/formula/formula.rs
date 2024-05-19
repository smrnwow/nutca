use super::tokens::Compound;
use crate::chemistry::{Element, Nutrient};
use crate::fertilizers::NutrientPercent;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Formula {
    elements: HashMap<Element, i32>,
}

impl Formula {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn from_compound(compound: Compound) -> Self {
        let mut elements: HashMap<Element, i32> = HashMap::new();

        compound.composition().iter().for_each(|element| {
            match elements.get_mut(&element.element()) {
                Some(elem) => {
                    *elem += element.subscript;
                }
                None => {
                    elements.insert(element.element(), element.subscript);
                }
            }
        });

        compound
            .hydrate()
            .iter()
            .for_each(|element| match elements.get_mut(&element.element()) {
                Some(elem) => {
                    *elem += element.subscript;
                }
                None => {
                    elements.insert(element.element(), element.subscript);
                }
            });

        if compound.coefficient() > 1 {
            elements.values_mut().for_each(|atoms_count| {
                *atoms_count *= compound.coefficient();
            });
        }

        Self { elements }
    }

    pub fn nutrients(&self) -> Vec<NutrientPercent> {
        let mut nutrients: Vec<NutrientPercent> = Vec::new();

        self.elements.iter().for_each(|(element, atoms_count)| {
            let atoms_count = *atoms_count as f64;

            let percent = (element.atomic_weight() * atoms_count) / self.molar_mass() * 100.;

            if let Some(nutrient) = element.nutrient() {
                if let Some(nutrient_percent) = match nutrient {
                    Nutrient::Nitrogen => Some(NutrientPercent::Nitrogen(percent)),
                    Nutrient::Phosphorus => Some(NutrientPercent::Phosphorus(percent)),
                    Nutrient::Potassium => Some(NutrientPercent::Potassium(percent)),
                    Nutrient::Calcium => Some(NutrientPercent::Calcium(percent)),
                    Nutrient::Magnesium => Some(NutrientPercent::Magnesium(percent)),
                    Nutrient::Sulfur => Some(NutrientPercent::Sulfur(percent)),
                    Nutrient::Iron => Some(NutrientPercent::Iron(percent)),
                    Nutrient::Zink => Some(NutrientPercent::Zink(percent)),
                    Nutrient::Manganese => Some(NutrientPercent::Manganese(percent)),
                    Nutrient::Boron => Some(NutrientPercent::Boron(percent)),
                    Nutrient::Copper => Some(NutrientPercent::Copper(percent)),
                    Nutrient::Molybdenum => Some(NutrientPercent::Molybdenum(percent)),
                    _ => None,
                } {
                    nutrients.push(nutrient_percent);
                }
            }
        });

        nutrients
    }

    fn molar_mass(&self) -> f64 {
        self.elements
            .iter()
            .map(|(element, atoms_count)| element.atomic_weight() * *atoms_count as f64)
            .sum()
    }
}
