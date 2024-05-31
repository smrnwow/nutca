use super::Element;
use crate::model::chemistry::Symbol;

#[derive(Debug)]
pub struct NitrogenForm {
    nitrogen_element: Option<Element>,
    nitrogen_atoms: i32,
    nitrate_form: i32,
    ammonium_form: i32,
}

impl NitrogenForm {
    pub fn new() -> Self {
        Self {
            nitrogen_element: None,
            nitrogen_atoms: 0,
            nitrate_form: 0,
            ammonium_form: 0,
        }
    }

    pub fn find(&mut self, element: Element) {
        match element.symbol() {
            Symbol::Nitrogen => {
                self.nitrogen_atoms += element.subscript();

                self.nitrogen_element = Some(element);
            }

            Symbol::Hydrogen => {
                if let Some(nitrogen) = &self.nitrogen_element {
                    if element.subscript() / nitrogen.subscript() == 4 {
                        self.ammonium_form += nitrogen.subscript();
                    }
                }

                self.nitrogen_element = None;
            }

            Symbol::Oxygen => {
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
}
