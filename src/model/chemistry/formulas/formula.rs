use super::{Compound, Element, NitrogenForm};
use crate::model::chemistry::{Nutrient, Symbol};
use crate::model::fertilizers::NutrientContent;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Formula {
    elements: HashMap<Symbol, i32>,
    nitrogen_form: NitrogenForm,
}

impl Formula {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            nitrogen_form: NitrogenForm::new(),
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

        let mut nitrogen_percent = 0.0;

        self.elements.iter().for_each(|(element, atoms_count)| {
            let atoms_count = *atoms_count as f64;

            let percent = (element.atomic_weight() * atoms_count) / self.molar_mass() * 100.;

            if let Some(nutrient) = element.nutrient() {
                if let Some(nutrient_percent) = match nutrient {
                    Nutrient::Nitrogen => {
                        nitrogen_percent = percent;

                        Some(NutrientContent::Nitrogen(percent))
                    }
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

        if nitrogen_percent > 0. {
            nutrients.push(NutrientContent::NitrogenNitrate(
                nitrogen_percent * (self.nitrogen_form.nitrate() / 100.),
            ));

            nutrients.push(NutrientContent::NitrogenAmmonium(
                nitrogen_percent * (self.nitrogen_form.ammonium() / 100.),
            ));
        }

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

        self.nitrogen_form.find(element.clone());
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

#[cfg(test)]
mod tests {
    use super::Formula;
    use crate::model::chemistry::formulas::Tokenizer;
    use crate::model::chemistry::Table;

    #[test]
    fn nitrogen_form() {
        let table = Table::new();

        // "KNO3"
        // "NH4NO3"
        // "Ca(NO3)2"
        // "2C14H18N3O10Fe(NH4)2"

        let formula = Formula::from_compound(Tokenizer::new(&table, "KH2PO4").tokenize().unwrap());

        println!("{:#?}", formula);

        println!("{:#?}", formula.nutrients());
    }
}
