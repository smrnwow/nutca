use crate::model::solutions::Solution;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionsListing {
    solutions: Vec<Solution>,
    search_query: String,
}

impl SolutionsListing {
    pub fn new(solutions: Vec<Solution>) -> Self {
        Self {
            solutions,
            search_query: String::new(),
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
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
}
