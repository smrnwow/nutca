use crate::repository::{ProfilesListing, Storage};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Dashboard {
    storage: Signal<Storage>,
    listing: ProfilesListing,
}

impl Dashboard {
    pub fn new(storage: Signal<Storage>) -> Self {
        let listing = ProfilesListing::new(storage);

        Self { storage, listing }
    }

    pub fn listing(&self) -> ProfilesListing {
        self.listing.clone()
    }

    pub fn search(&mut self, search_query: String) {
        self.listing.search(search_query);
    }

    pub fn add(&mut self) {
        navigator().push(Route::ProfileAddPage {});
    }

    pub fn edit(&mut self, profile_id: String) {
        navigator().push(Route::ProfileEditPage { profile_id });
    }

    pub fn add_solution(&mut self, profile_id: String) {
        navigator().push(Route::SolutionAddPage { profile_id });
    }

    pub fn delete(&mut self, profile_id: String) {
        match self.storage.read().profiles().delete(profile_id) {
            Ok(_) => {
                self.listing = ProfilesListing::new(self.storage);
            }

            Err(_) => println!("error"),
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.listing.paginate(page_index);
    }
}
