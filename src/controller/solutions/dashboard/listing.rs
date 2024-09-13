use crate::model::solutions::SolutionSummary;
use crate::repository::SolutionsRepository;

#[derive(Clone, Debug, PartialEq)]
pub struct Listing {
    solutions_repository: SolutionsRepository,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl Listing {
    pub fn new(solutions_repository: SolutionsRepository) -> Self {
        Self {
            solutions_repository,
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

    pub fn fetch(&self) -> Vec<SolutionSummary> {
        self.solutions_repository
            .search(&self.search_query, self.limit, self.page_index)
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
