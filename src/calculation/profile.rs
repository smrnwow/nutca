use super::composition::Composition;
use crate::fertilizers::fertilizer::Fertiliser;
use std::slice::Iter;

pub struct Profile<'a> {
    composition: Composition<'a>,
    fertilizers: Vec<Fertiliser>,
}

impl<'a> Profile<'a> {
    pub fn new(composition: Composition<'a>) -> Self {
        Self {
            composition,
            fertilizers: Vec::new(),
        }
    }

    pub fn add_fertilizer(&mut self, fertilizer: Fertiliser) {
        self.fertilizers.push(fertilizer);
    }

    pub fn fertilizers(&self) -> Iter<'_, Fertiliser> {
        self.fertilizers.iter()
    }

    pub fn composition(&self) -> &Composition<'a> {
        &self.composition
    }
}
