use crate::model::profiles::Profile;
use crate::repository::{Error, Storage};
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ProfilesRepository {
    storage: Signal<Storage>,
}

impl ProfilesRepository {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self { storage }
    }

    pub fn find_or_default(&self, profile_id: &str) -> Profile {
        match self.storage.read().profiles().get(profile_id) {
            Ok(profile) => profile,
            Err(_) => Profile::new(),
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

    pub fn create(&self, profile: Profile) -> Result<(), Error> {
        let result = self.storage.read().profiles().add(profile);

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn update(&self, profile: Profile) -> Result<(), Error> {
        let result = self.storage.read().profiles().update(profile);

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}
