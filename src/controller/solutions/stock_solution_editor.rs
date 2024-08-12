use crate::controller::solutions::SolutionsListing;
use crate::model::chemistry::Volume;
use crate::model::solutions::{FertilizerWeight, Solution, StockSolutionBuilder};
use crate::repository::Storage;
use dioxus::prelude::*;

pub struct StockSolutionEditor {
    builder: StockSolutionBuilder,
    solution: Solution,
    solutions_listing: SolutionsListing,
}

impl StockSolutionEditor {
    pub fn new(storage: Signal<Storage>, solution_id: String) -> Self {
        let solution = storage.read().solutions().get(&solution_id);

        let builder = match solution {
            Ok(solution) => StockSolutionBuilder::from(solution),
            Err(_) => StockSolutionBuilder::new(),
        };

        let solutions_listing = SolutionsListing::new(storage);

        let solution = builder.solution().clone();

        Self {
            builder,
            solutions_listing,
            solution,
        }
    }

    pub fn list_solutions(&self) -> SolutionsListing {
        self.solutions_listing.clone()
    }

    pub fn solution(&self) -> &Solution {
        &self.solution
    }

    pub fn concentration(&self) -> usize {
        self.builder.concentration_factor()
    }

    pub fn volume(&self) -> Volume {
        self.builder.volume()
    }

    pub fn part_a(&self) -> Vec<FertilizerWeight> {
        self.builder.part_a()
    }

    pub fn part_b(&self) -> Vec<FertilizerWeight> {
        self.builder.part_b()
    }

    pub fn search_solution(&mut self, search_query: String) {
        self.solutions_listing.search(search_query);
    }

    pub fn change_solution(&mut self, solution_id: String) {
        let solution = self.solutions_listing.find(solution_id);
        self.builder.update_solution(solution);
    }

    pub fn change_concentration(&mut self, concentration: usize) {
        self.builder.update_concentration_factor(concentration);
    }

    pub fn change_volume(&mut self, volume: Volume) {
        self.builder.update_volume(volume);
    }
}
