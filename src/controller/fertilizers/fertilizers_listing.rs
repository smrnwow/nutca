use crate::model::fertilizers::Fertilizer;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersListing {
    storage: Signal<Storage>,
    excluded_ids: Vec<String>,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl FertilizersListing {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            excluded_ids: Vec::new(),
            search_query: String::new(),
            page_index: 1,
            limit: 10,
        }
    }

    pub fn fetch(&self) -> Vec<Fertilizer> {
        match self.storage.read().fertilizers().search(
            &self.search_query,
            &[],
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }

    pub fn refresh(&mut self) {}

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }
}
