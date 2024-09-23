use crate::controller::profiles::ProfilesListing;
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Dashboard {
    storage: Signal<Storage>,
    listing: Signal<ProfilesListing>,
}

impl Dashboard {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            listing: Signal::new(ProfilesListing::new(storage)),
        }
    }

    pub fn listing(&self) -> Signal<ProfilesListing> {
        self.listing
    }

    pub fn search(&mut self, search_query: String) {
        self.listing.write().search(search_query);
    }

    pub fn add(&mut self) {
        navigator().push(Route::ProfileAddPage {});
    }

    pub fn edit(&mut self, profile_id: String) {
        navigator().push(Route::ProfileEditPage { profile_id });
    }

    pub fn add_solution(&mut self, profile_id: String) {
        navigator().push(Route::SolutionAddPage {
            profile_id,
            concentrate_id: String::new(),
        });
    }

    pub fn delete(&mut self, profile_id: String) {
        match self.storage.read().profiles().delete(profile_id) {
            Ok(_) => self.listing.write().refresh(),
            Err(_) => println!("error"),
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.listing.write().paginate(page_index);
    }
}
