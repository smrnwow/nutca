use crate::model::solutions::{Solution, SolutionSummary};
use crate::repository::SolutionsRepository;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionsBrowser {
    solutions_repository: SolutionsRepository,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl SolutionsBrowser {
    pub fn new(solutions_repository: SolutionsRepository) -> Self {
        Self {
            solutions_repository,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
        }
    }

    pub fn select(&mut self, solution_id: &String) -> Solution {
        self.solutions_repository.get(solution_id)
    }

    pub fn options(&self) -> Vec<(String, String)> {
        self.fetch()
            .iter()
            .map(|solution| (solution.id().clone(), solution.name().clone()))
            .collect()
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn fetch(&self) -> Vec<SolutionSummary> {
        self.solutions_repository
            .search(&self.search_query, self.limit, self.page_index)
    }
}
