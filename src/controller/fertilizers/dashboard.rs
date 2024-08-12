use crate::controller::fertilizers::FertilizersListing;
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Dashboard {
    storage: Signal<Storage>,
    listing: Signal<FertilizersListing>,
}

impl Dashboard {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            listing: Signal::new(FertilizersListing::new(storage)),
        }
    }

    pub fn listing(&self) -> Signal<FertilizersListing> {
        self.listing
    }

    pub fn search(&mut self, search_query: String) {
        self.listing.write().search(search_query);
    }

    pub fn add(&mut self) {
        navigator().push(Route::FertilizerAddPage {});
    }

    pub fn edit(&mut self, fertilizer_id: String) {
        navigator().push(Route::FertilizerEditPage { fertilizer_id });
    }

    pub fn delete(&mut self, fertilizer_id: String) {
        match self.storage.read().fertilizers().delete(fertilizer_id) {
            Ok(_) => self.listing.write().refresh(),
            Err(_) => println!("error"),
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.listing.write().paginate(page_index);
    }
}
