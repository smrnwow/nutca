use super::element::Element;

#[derive(Debug)]
pub struct Hydrate {
    coefficient: i32,
    elements: Vec<Element>,
    empty: bool,
}

impl Hydrate {
    pub fn from(coefficient: i32, elements: Vec<Element>) -> Self {
        Self {
            coefficient,
            elements,
            empty: false,
        }
    }

    pub fn new() -> Self {
        Self {
            coefficient: 1,
            elements: vec![],
            empty: true,
        }
    }

    pub fn add_coefficient(&mut self, coefficient: i32) {
        self.coefficient = coefficient;

        self.empty = false;
    }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);

        self.empty = false;
    }

    pub fn empty(&self) -> bool {
        self.empty
    }

    pub fn elements(&self) -> Vec<Element> {
        let mut elements = Vec::new();

        self.elements.iter().for_each(|element| {
            elements.push(Element::multiply(element, self.coefficient));
        });

        elements
    }
}

impl PartialEq for Hydrate {
    fn eq(&self, other: &Self) -> bool {
        if self.coefficient != other.coefficient {
            return false;
        }

        if self.elements.len() != other.elements.len() {
            return false;
        }

        if !self
            .elements
            .iter()
            .zip(&other.elements)
            .all(|(this, other)| this == other)
        {
            return false;
        }

        return true;
    }
}
