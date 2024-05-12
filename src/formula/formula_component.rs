use crate::chemistry::element::Element;

#[derive(Debug)]
pub struct FormulaComponent {
    element: Element,
    atoms_count: i32,
    mass_percent: f32,
}

impl FormulaComponent {
    pub fn new(element: Element, atoms_count: i32) -> Self {
        Self {
            element,
            atoms_count,
            mass_percent: 0.0,
        }
    }

    pub fn increment_atoms_count(&mut self, atoms_count: i32) {
        self.atoms_count += atoms_count;
    }

    pub fn multiple_atoms_count(&mut self, coefficient: i32) {
        self.atoms_count = self.atoms_count * coefficient;
    }

    pub fn calculate_molar_mass(&self) -> f32 {
        self.element.atomic_weight * self.atoms_count as f32
    }

    pub fn calculate_mass_percent(&mut self, compound_molar_mass: f32) {
        self.mass_percent = (self.calculate_molar_mass() / compound_molar_mass) * 100.0
    }

    pub fn mass_percent(&self) -> f32 {
        self.mass_percent
    }

    pub fn element(&self) -> &Element {
        &self.element
    }
}
