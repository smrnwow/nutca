use crate::model::profiles::Profile;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NutritionProgramBrowser {
    storage: Signal<Storage>,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl NutritionProgramBrowser {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
        }
    }

    pub fn fetch(&self) -> Vec<Profile> {
        match self.storage.read().profiles().search(
            &self.search_query,
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(profiles) => profiles,
            Err(_) => Vec::new(),
        }
    }

    pub fn find(&self, profile_id: String) -> Option<Profile> {
        match self.storage.read().profiles().get(&profile_id) {
            Ok(profile) => Some(profile),
            Err(_) => None,
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }
}
