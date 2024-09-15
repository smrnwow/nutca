use crate::model::concentrates::ConcentrateSummary;
use crate::repository::ConcentratesRepository;

#[derive(Clone, Debug, PartialEq)]
pub struct Listing {
    concentrates_repository: ConcentratesRepository,
    search_query: String,
    limit: usize,
    page_index: usize,
}

impl Listing {
    pub fn new(concentrates_repository: ConcentratesRepository) -> Self {
        Self {
            concentrates_repository,
            search_query: String::new(),
            limit: 10,
            page_index: 1,
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn refresh(&mut self) {}

    pub fn fetch(&self) -> Vec<ConcentrateSummary> {
        self.concentrates_repository
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
