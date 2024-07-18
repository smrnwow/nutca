use crate::model::chemistry::{NutrientAmount, Nutrients};
use crate::model::fertilizers::formulas::NitrogenForms;
use chemp::ChemicalElement;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Formula {
    pub nutrients: Nutrients,
    formulation: String,
    error: Option<String>,
}

impl Formula {
    pub fn new(formulation: impl Into<String>) -> Self {
        Self {
            formulation: formulation.into(),
            nutrients: Nutrients::new(),
            error: None,
        }
    }

    pub fn formulation(&self) -> String {
        self.formulation.clone()
    }

    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }

    fn set_error(&mut self, error: String) {
        self.error = Some(error);
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

        let mut total_nitrogen_percent: f64 = 0.0;

        match chemp::parse(formulation) {
            Ok(compound) => {
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
                        formula.nutrients.set(nutrient);
                    }

                    if let ChemicalElement::Nitrogen = component.chemical_element() {
                        total_nitrogen_percent = percent;
                    }
                });

                let mut nitrogen_forms = NitrogenForms::new();

                compound.composition().iter().for_each(|element| {
                    nitrogen_forms.find(*element);
                });

                nitrogen_forms
                    .values(total_nitrogen_percent)
                    .iter()
                    .for_each(|nitrogen_form| {
                        formula.nutrients.set(*nitrogen_form);
                    });
            }

            Err(error) => formula.set_error(error.to_string()),
        }

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

        println!("{:#?}", formula.nutrients);
    }
}
