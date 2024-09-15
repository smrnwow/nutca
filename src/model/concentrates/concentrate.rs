use super::{Composition, DefaultDistribution, Part};
use crate::model::solutions::Solution;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Concentrate {
    id: String,
    name: String,
    composition: Composition,
    parts: Vec<Part>,
}

impl Concentrate {
    pub fn new(id: String, name: String, composition: Composition, parts: Vec<Part>) -> Self {
        Self {
            id,
            name,
            composition,
            parts,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn composition(&self) -> &Composition {
        &self.composition
    }

    pub fn composition_mut(&mut self) -> &mut Composition {
        &mut self.composition
    }

    pub fn set_composition(&mut self, composition: Composition) {
        self.composition = composition;
    }

    pub fn parts(&self) -> Vec<&Part> {
        self.parts.iter().collect()
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn change_solution(&mut self, solution: Solution) {
        if let Composition::FromSolution(_) = &mut self.composition {
            let (composition, parts) = DefaultDistribution::new(solution).distribute();

            self.composition = Composition::FromSolution(composition);

            self.parts = parts;
        }
    }

    pub fn add_part(&mut self) {
        if self.parts.len() < 5 {
            self.parts.push(Part::new(""));
        }
    }

    pub fn get_part(&mut self, part_id: &String) -> Option<&mut Part> {
        let position = self.parts.iter().position(|part| *part.id() == *part_id);

        match position {
            Some(index) => self.parts.get_mut(index),
            None => None,
        }
    }

    pub fn remove_part(&mut self, part_id: &String) {
        let position = self.parts.iter().position(|part| *part.id() == *part_id);

        if let Some(index) = position {
            self.parts.remove(index);

            self.composition.remove_part(part_id);
        }
    }
}

impl From<Solution> for Concentrate {
    fn from(solution: Solution) -> Self {
        let (composition, parts) = DefaultDistribution::new(solution).distribute();

        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            composition: Composition::FromSolution(composition),
            parts,
        }
    }
}

impl Default for Concentrate {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            composition: Composition::default(),
            parts: Vec::new(),
        }
    }
}
