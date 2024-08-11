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
            limit: 8,
        }
    }

    pub fn fetch(&self) -> Vec<Fertilizer> {
        match self.storage.read().fertilizers().search(
            self.search_query.clone(),
            self.excluded_ids.as_ref(),
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }

    pub fn find(&self, fertilizer_id: String) -> Option<Fertilizer> {
        match self.storage.read().fertilizers().get(fertilizer_id) {
            Ok(fertilizer) => Some(fertilizer),
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

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn exclude_many(&mut self, fertilizers_ids: Vec<String>) {
        for fertilizer_id in fertilizers_ids {
            self.excluded_ids.push(fertilizer_id);
        }
    }

    pub fn exclude(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        self.excluded_ids.push(fertilizer_id.clone());

        self.find(fertilizer_id)
    }

    pub fn include(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        match self.excluded_ids.iter().position(|id| *id == fertilizer_id) {
            Some(index) => {
                self.excluded_ids.remove(index);

                self.find(fertilizer_id)
            }

            None => None,
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn update_limit(&mut self, limit: usize) {
        self.limit = limit;
    }
}
