use crate::model::fertilizers::Fertilizer;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersBrowser {
    storage: Signal<Storage>,
    search_query: String,
    limit: usize,
    page_index: usize,
}

impl FertilizersBrowser {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            search_query: String::new(),
            limit: 8,
            page_index: 1,
        }
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub(super) fn filter(&self, fertilizers_ids: Vec<&String>) -> Vec<Fertilizer> {
        let fertilizers_ids: Vec<String> = fertilizers_ids.into_iter().cloned().collect();

        match self.storage.read().fertilizers().filter(&fertilizers_ids) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }

    pub(super) fn fetch(&self, projection: Vec<&String>) -> Vec<Fertilizer> {
        match self.storage.read().fertilizers().search(
            &self.search_query,
            &projection,
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }

    pub(super) fn find(&self, fertilizer_id: &str) -> Option<Fertilizer> {
        match self.storage.read().fertilizers().get(fertilizer_id) {
            Ok(fertilizer) => Some(fertilizer),
            Err(_) => None,
        }
    }
}
