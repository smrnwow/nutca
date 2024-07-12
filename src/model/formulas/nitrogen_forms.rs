use crate::model::chemistry::Nutrient;
use chemp::{ChemicalElement, Element};

#[derive(Clone, Debug, PartialEq)]
pub struct NitrogenForms {
    nitrogen_element: Option<Element>,
    nitrogen_atoms: usize,
    nitrate_form: usize,
    ammonium_form: usize,
}

impl NitrogenForms {
    pub fn new() -> Self {
        Self {
            nitrogen_element: None,
            nitrogen_atoms: 0,
            nitrate_form: 0,
            ammonium_form: 0,
        }
    }

    pub fn find(&mut self, element: Element) {
        match element.chemical_element() {
            ChemicalElement::Nitrogen => {
                self.nitrogen_atoms += element.subscript();

                self.nitrogen_element = Some(element);
            }

            ChemicalElement::Hydrogen => {
                if let Some(nitrogen) = &self.nitrogen_element {
                    if element.subscript() / nitrogen.subscript() == 4 {
                        self.ammonium_form += nitrogen.subscript();
                    }
                }

                self.nitrogen_element = None;
            }

            ChemicalElement::Oxygen => {
                if let Some(nitrogen) = &self.nitrogen_element {
                    if element.subscript() / nitrogen.subscript() == 3 {
                        self.nitrate_form += nitrogen.subscript();
                    }
                }

                self.nitrogen_element = None;
            }

            _ => {
                self.nitrogen_element = None;
            }
        }
    }

    pub fn nitrate(&self) -> f64 {
        (self.nitrate_form as f64) / (self.nitrogen_atoms as f64) * 100.
    }

    pub fn ammonium(&self) -> f64 {
        (self.ammonium_form as f64) / (self.nitrogen_atoms as f64) * 100.
    }

    pub fn values(&self, total_nitrogen_percent: f64) -> Vec<Nutrient> {
        if total_nitrogen_percent > 0. {
            let mut nitrogen_forms = vec![];

            nitrogen_forms.push(Nutrient::NitrogenNitrate(
                total_nitrogen_percent * (self.nitrate() / 100.),
            ));

            nitrogen_forms.push(Nutrient::NitrogenAmmonium(
                total_nitrogen_percent * (self.ammonium() / 100.),
            ));

            nitrogen_forms
        } else {
            vec![
                Nutrient::NitrogenNitrate(0.0),
                Nutrient::NitrogenAmmonium(0.0),
            ]
        }
    }
}
