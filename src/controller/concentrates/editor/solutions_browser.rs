use super::PickedSolution;
use crate::model::solutions::Solution;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionsBrowser {
    storage: Signal<Storage>,
    search_query: String,
    page_index: usize,
    limit: usize,
    picked_solution: PickedSolution,
}

impl SolutionsBrowser {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
            picked_solution: PickedSolution::new(storage),
        }
    }

    pub fn select(&mut self, solution_id: &String) {
        match self.storage.read().solutions().get(solution_id) {
            Ok(solution) => self.picked_solution.change(solution),
            Err(_) => {
                // maybe notification here
            }
        }
    }

    pub fn picked_solution(&self) -> &PickedSolution {
        &self.picked_solution
    }

    pub fn value(&self) -> (String, String) {
        (self.picked_solution.id(), self.picked_solution.name())
    }

    pub fn options(&self) -> Vec<(String, String)> {
        self.fetch()
            .iter()
            .map(|solution| (solution.id(), solution.name()))
            .collect()
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn fetch(&self) -> Vec<Solution> {
        match self.storage.read().solutions().search(
            &self.search_query,
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(solutions) => solutions,
            Err(_) => Vec::new(),
        }
    }
}
