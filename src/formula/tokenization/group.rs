use super::component::Component;
use super::element::Element;

#[derive(Debug)]
pub struct Group {
    composition: Vec<Component>,
    subscript: i32,
}

impl Group {
    pub fn from(composition: Vec<Component>, subscript: i32) -> Self {
        Self {
            composition,
            subscript,
        }
    }

    pub fn new() -> Self {
        Self {
            composition: vec![],
            subscript: 1,
        }
    }

    pub fn add_element(&mut self, element: Element) {
        self.composition.push(Component::Element(element));
    }

    pub fn add_group(&mut self, group: Group) {
        self.composition.push(Component::Group(group));
    }

    pub fn add_subscript(&mut self, subscript: i32) {
        self.subscript = subscript;
    }

    pub fn elements(&self) -> Vec<Element> {
        let mut elements = vec![];

        self.composition
            .iter()
            .for_each(|component| match component {
                Component::Element(element) => {
                    elements.push(Element::multiply(element, self.subscript));
                }
                Component::Group(group) => {
                    group.elements().iter().for_each(|element| {
                        elements.push(Element::multiply(element, self.subscript));
                    });
                }
            });

        elements
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Self) -> bool {
        if self.composition.len() != other.composition.len() {
            return false;
        }

        if !self
            .composition
            .iter()
            .zip(&other.composition)
            .all(|(this, other)| this == other)
        {
            return false;
        }

        if self.subscript != other.subscript {
            return false;
        }

        true
    }
}
