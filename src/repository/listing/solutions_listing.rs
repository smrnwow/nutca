use crate::model::solutions::Solution;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionsListing {
    solutions: Vec<Solution>,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl SolutionsListing {
    pub fn new(solutions: Vec<Solution>) -> Self {
        Self {
            solutions,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn find(&self, solution_id: String) -> Option<Solution> {
        let solution = self
            .solutions
            .iter()
            .find(|solution| solution.id() == solution_id);

        match solution {
            Some(solution) => Some(solution.clone()),
            None => None,
        }
    }

    pub fn list(&self) -> Vec<Solution> {
        if self.search_query.len() == 0 {
            self.solutions.clone()
        } else {
            self.solutions
                .iter()
                .filter(|solution| {
                    solution
                        .name()
                        .to_lowercase()
                        .contains(self.search_query.as_str())
                })
                .map(|solution| solution.clone())
                .collect()
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn total(&self) -> usize {
        self.solutions
            .iter()
            .filter(|solution| {
                solution
                    .name()
                    .to_lowercase()
                    .contains(self.search_query.as_str())
            })
            .collect::<Vec<&Solution>>()
            .len()
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }
}
