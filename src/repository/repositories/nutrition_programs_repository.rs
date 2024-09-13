use crate::model::profiles::Profile;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct NutritionProgramsRepository {
    storage: Signal<Storage>,
}

impl NutritionProgramsRepository {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self { storage }
    }

    pub fn find_or_default(&self, nutrition_program_id: &String) -> Profile {
        match self.storage.read().profiles().get(nutrition_program_id) {
            Ok(profile) => profile,
            Err(_) => Profile::default(),
        }
    }

    pub fn search(&self, query: &String, limit: usize, page_index: usize) -> Vec<Profile> {
        match self
            .storage
            .read()
            .profiles()
            .search(query, limit, limit * (page_index - 1))
        {
            Ok(profiles) => profiles,
            Err(_) => Vec::new(),
        }
    }

    pub fn find(&self, profile_id: &String) -> Option<Profile> {
        match self.storage.read().profiles().get(profile_id) {
            Ok(profile) => Some(profile),
            Err(_) => None,
        }
    }
}
