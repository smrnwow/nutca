use crate::repository::{FertilizersListing, Storage};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Dashboard {
    storage: Signal<Storage>,
    listing: FertilizersListing,
}

impl Dashboard {
    pub fn new(storage: Signal<Storage>) -> Self {
        let mut listing = FertilizersListing::new(storage);

        listing.update_limit(10);

        Self { storage, listing }
    }

    pub fn listing(&self) -> FertilizersListing {
        self.listing.clone()
    }

    pub fn search(&mut self, search_query: String) {
        self.listing.search(search_query);
    }

    pub fn add(&mut self) {
        navigator().push(Route::FertilizerAddPage {});
    }

    pub fn edit(&mut self, fertilizer_id: String) {
        navigator().push(Route::FertilizerEditPage { fertilizer_id });
    }

    pub fn delete(&mut self, fertilizer_id: String) {
        match self.storage.read().fertilizers().delete(fertilizer_id) {
            Ok(_) => {
                self.listing = FertilizersListing::new(self.storage);
            }

            Err(_) => println!("error"),
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.listing.paginate(page_index);
    }
}
