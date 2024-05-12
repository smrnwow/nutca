use super::formula_component::FormulaComponent;
use crate::chemistry::element::Element;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Formula {
    elements: HashMap<&'static str, FormulaComponent>,
    molar_mass: f32,
}

impl Formula {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            molar_mass: 0.0,
        }
    }

    pub fn add_element(&mut self, element: Element, atoms_count: i32) {
        match self.elements.get_mut(&element.symbol) {
            Some(formula_component) => {
                formula_component.increment_atoms_count(atoms_count);
            }
            None => {
                self.elements
                    .insert(&element.symbol, FormulaComponent::new(element, atoms_count));
            }
        }
    }

    pub fn multiple(&mut self, coefficient: i32) {
        self.elements
            .values_mut()
            .for_each(|formula_component| formula_component.multiple_atoms_count(coefficient));
    }

    pub fn calculate_molar_mass(&mut self) {
        self.elements.values().for_each(|formula_component| {
            self.molar_mass += formula_component.calculate_molar_mass();
        });
    }

    pub fn calculate_mass_percent(&mut self) {
        self.elements.values_mut().for_each(|formula_component| {
            formula_component.calculate_mass_percent(self.molar_mass);
        });
    }

    pub fn components(&self) -> Vec<&FormulaComponent> {
        self.elements.values().collect()
    }

    pub fn molar_mass(&self) -> f32 {
        self.molar_mass
    }
}
