use super::{Composition, CompositionFromSolution, Part};
use crate::model::solutions::Solution;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Concentrate {
    id: String,
    name: String,
    composition: Composition,
}

impl Concentrate {
    pub fn new(id: String, name: String, composition: Composition) -> Self {
        Self {
            id,
            name,
            composition,
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
        self.composition.parts()
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn change_solution(&mut self, solution: Solution) {
        if let Composition::FromSolution(_) = &mut self.composition {
            let composition = CompositionFromSolution::from(solution);

            self.composition = Composition::FromSolution(composition);
        }
    }

    pub fn add_part(&mut self) {
        self.composition.add_part(Part::new(""));
    }

    pub fn get_part(&mut self, part_id: &String) -> Option<&mut Part> {
        self.composition.get_part(part_id)
    }

    pub fn remove_part(&mut self, part_id: &String) {
        self.composition.remove_part(part_id)
    }
}

impl From<Solution> for Concentrate {
    fn from(solution: Solution) -> Self {
        let composition = CompositionFromSolution::from(solution);

        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            composition: Composition::FromSolution(composition),
        }
    }
}

impl Default for Concentrate {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            composition: Composition::default(),
        }
    }
}
