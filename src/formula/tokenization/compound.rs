use super::component::Component;
use super::element::Element;
use super::group::Group;
use super::hydrate::Hydrate;

#[derive(Debug)]
pub struct Compound {
    coefficient: i32,
    composition: Vec<Component>,
    hydrate: Option<Hydrate>,
    empty: bool,
}

impl Compound {
    pub fn new() -> Self {
        Self {
            coefficient: 1,
            composition: vec![],
            hydrate: None,
            empty: true,
        }
    }

    pub fn from(coefficient: i32, composition: Vec<Component>, hydrate: Option<Hydrate>) -> Self {
        Self {
            coefficient,
            composition,
            hydrate,
            empty: false,
        }
    }

    pub fn add_element(&mut self, element: Element) {
        self.composition.push(Component::Element(element));
        self.empty = false;
    }

    pub fn add_group(&mut self, group: Group) {
        self.composition.push(Component::Group(group));
        self.empty = false;
    }

    pub fn add_coefficient(&mut self, coefficient: i32) {
        self.coefficient = coefficient;
        self.empty = false;
    }

    pub fn add_hydrate(&mut self, hydrate: Hydrate) {
        self.hydrate = Some(hydrate);
        self.empty = false;
    }

    pub fn elements(&self) -> Vec<Element> {
        let mut elements: Vec<Element> = vec![];

        self.composition
            .iter()
            .for_each(|component| match component {
                Component::Element(element) => {
                    elements.push(element.clone());
                }
                Component::Group(group) => {
                    group.elements().iter().for_each(|element| {
                        elements.push(element.clone());
                    });
                }
            });

        if let Some(hydrate) = &self.hydrate {
            hydrate.elements().iter().for_each(|element| {
                elements.push(element.clone());
            });
        }

        elements
    }

    pub fn coefficient(&self) -> i32 {
        self.coefficient
    }

    pub fn empty(&self) -> bool {
        self.empty
    }
}

impl PartialEq for Compound {
    fn eq(&self, other: &Self) -> bool {
        if self.coefficient != other.coefficient {
            return false;
        }

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

        if self.hydrate != other.hydrate {
            return false;
        }

        true
    }
}
