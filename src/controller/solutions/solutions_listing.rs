use crate::model::solutions::Solution;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionsListing {
    storage: Signal<Storage>,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl SolutionsListing {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn refresh(&mut self) {}

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

    pub fn find(&self, solution_id: String) -> Option<Solution> {
        match self.storage.read().solutions().get(&solution_id) {
            Ok(solution) => Some(solution),
            Err(_) => None,
        }
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }
}
