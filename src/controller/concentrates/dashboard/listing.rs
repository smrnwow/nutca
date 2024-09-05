use crate::model::concentrates::Concentrate;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Listing {
    storage: Signal<Storage>,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl Listing {
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

    pub fn fetch(&self) -> Vec<Concentrate> {
        match self.storage.read().concentrates().search(
            &self.search_query,
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(concentrates) => concentrates,
            Err(_) => Vec::new(),
        }
    }

    pub fn find(&self, concentrate_id: String) -> Option<Concentrate> {
        match self.storage.read().concentrates().get(&concentrate_id) {
            Ok(concentrate) => Some(concentrate),
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
