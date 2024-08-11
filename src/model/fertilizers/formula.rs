use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use crate::model::fertilizers::NitrogenForms;
use chemp::{ChemicalElement, Compound};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Formula {
    nutrients: Nutrients,
    formulation: String,
    error: Option<String>,
}

impl Formula {
    pub fn formulation(&self) -> String {
        self.formulation.clone()
    }

    pub fn nutrients(&self) -> Nutrients {
        self.nutrients
    }

    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }

    fn set_nutrients(&mut self, compound: &Compound) {
        compound.components().values().for_each(|component| {
            let percent = component.mass_percent() as f64;

            if let Some(nutrient) = match component.chemical_element() {
                ChemicalElement::Nitrogen => Some(NutrientAmount::Nitrogen(percent)),
                ChemicalElement::Phosphorus => Some(NutrientAmount::Phosphorus(percent)),
                ChemicalElement::Potassium => Some(NutrientAmount::Potassium(percent)),
                ChemicalElement::Calcium => Some(NutrientAmount::Calcium(percent)),
                ChemicalElement::Magnesium => Some(NutrientAmount::Magnesium(percent)),
                ChemicalElement::Sulfur => Some(NutrientAmount::Sulfur(percent)),
                ChemicalElement::Iron => Some(NutrientAmount::Iron(percent)),
                ChemicalElement::Zinc => Some(NutrientAmount::Zinc(percent)),
                ChemicalElement::Manganese => Some(NutrientAmount::Manganese(percent)),
                ChemicalElement::Boron => Some(NutrientAmount::Boron(percent)),
                ChemicalElement::Copper => Some(NutrientAmount::Copper(percent)),
                ChemicalElement::Molybdenum => Some(NutrientAmount::Molybdenum(percent)),
                _ => None,
            } {
                self.nutrients.set(nutrient);
            }
        });
    }

    fn set_nitrogen_forms(&mut self, compound: &Compound) {
        let total_nitrogen = self.nutrients.value_of(Nutrient::Nitrogen);

        let mut nitrogen_forms = NitrogenForms::new();

        compound.composition().iter().for_each(|element| {
            nitrogen_forms.find(*element);
        });

        nitrogen_forms
            .values(total_nitrogen.value())
            .iter()
            .for_each(|nitrogen_form| {
                self.nutrients.set(*nitrogen_form);
            });
    }

    fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }
}

impl Default for Formula {
    fn default() -> Self {
        Self {
            formulation: String::new(),
            nutrients: Nutrients::new(),
            error: None,
        }
    }
}

impl From<&str> for Formula {
    fn from(formulation: &str) -> Self {
        Self::from(formulation.to_string())
    }
}

impl From<String> for Formula {
    fn from(formulation: String) -> Self {
        let mut formula = Self {
            formulation,
            nutrients: Nutrients::new(),
            error: None,
        };

        match chemp::parse(formula.formulation.as_str()) {
            Ok(compound) => {
                formula.set_nutrients(&compound);
                formula.set_nitrogen_forms(&compound);
            }

            Err(error) => formula.set_error(error.to_string()),
        }

        formula
    }
}

#[cfg(test)]
mod tests {}
