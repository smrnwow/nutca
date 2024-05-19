use crate::chemistry;

#[derive(Debug, Clone)]
pub struct Element {
    pub element: chemistry::Element,
    pub subscript: i32,
}

impl Element {
    pub fn new(element: chemistry::Element, subscript: i32) -> Self {
        Self { element, subscript }
    }

    pub fn from(symbol: &str, subscript: i32) -> Self {
        let table = chemistry::Table::new();

        Self {
            element: table.by_symbol(symbol).unwrap(),
            subscript,
        }
    }

    pub fn multiply(element: &Self, coefficient: i32) -> Self {
        Self {
            element: element.element.clone(),
            subscript: element.subscript * coefficient,
        }
    }

    pub fn add_element(&mut self, element: chemistry::Element) {
        self.element = element;
    }

    pub fn add_subscript(&mut self, subscript: i32) {
        self.subscript = subscript;
    }

    pub fn element(&self) -> chemistry::Element {
        self.element
    }

    pub fn molar_mass(&self) -> f64 {
        self.element.atomic_weight() * self.subscript as f64
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        if self.element != other.element {
            return false;
        }

        if self.subscript != other.subscript {
            return false;
        }

        true
    }
}
