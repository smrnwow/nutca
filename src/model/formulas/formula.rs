use super::NitrogenForms;
use crate::model::chemistry::Nutrient;
use chemp::ChemicalElement;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Formula {
    formulation: String,
    nutrients: [Nutrient; 14],
    error: Option<String>,
}

impl Formula {
    pub fn new(formulation: impl Into<String>) -> Self {
        Self {
            formulation: formulation.into(),
            nutrients: [
                Nutrient::Nitrogen(0.0),
                Nutrient::NitrogenNitrate(0.0),
                Nutrient::NitrogenAmmonium(0.0),
                Nutrient::Phosphorus(0.0),
                Nutrient::Potassium(0.0),
                Nutrient::Calcium(0.0),
                Nutrient::Magnesium(0.0),
                Nutrient::Sulfur(0.0),
                Nutrient::Iron(0.0),
                Nutrient::Zinc(0.0),
                Nutrient::Manganese(0.0),
                Nutrient::Boron(0.0),
                Nutrient::Copper(0.0),
                Nutrient::Molybdenum(0.0),
            ],
            error: None,
        }
    }

    pub fn nutrients(&self) -> Vec<Nutrient> {
        Vec::from(self.nutrients)
    }

    pub fn formulation(&self) -> String {
        self.formulation.clone()
    }

    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }

    fn set_nutrient(&mut self, nutrient: Nutrient) {
        self.nutrients[nutrient.index()] = nutrient;
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
                        ChemicalElement::Nitrogen => Some(Nutrient::Nitrogen(percent)),
                        ChemicalElement::Phosphorus => Some(Nutrient::Phosphorus(percent)),
                        ChemicalElement::Potassium => Some(Nutrient::Potassium(percent)),
                        ChemicalElement::Calcium => Some(Nutrient::Calcium(percent)),
                        ChemicalElement::Magnesium => Some(Nutrient::Magnesium(percent)),
                        ChemicalElement::Sulfur => Some(Nutrient::Sulfur(percent)),
                        ChemicalElement::Iron => Some(Nutrient::Iron(percent)),
                        ChemicalElement::Zinc => Some(Nutrient::Zinc(percent)),
                        ChemicalElement::Manganese => Some(Nutrient::Manganese(percent)),
                        ChemicalElement::Boron => Some(Nutrient::Boron(percent)),
                        ChemicalElement::Copper => Some(Nutrient::Copper(percent)),
                        ChemicalElement::Molybdenum => Some(Nutrient::Molybdenum(percent)),
                        _ => None,
                    } {
                        formula.set_nutrient(nutrient);
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
                        formula.set_nutrient(*nitrogen_form);
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

        println!("{:#?}", formula.nutrients());
    }
}
