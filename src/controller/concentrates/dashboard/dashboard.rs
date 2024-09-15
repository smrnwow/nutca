use super::Listing;
use crate::repository::{ConcentratesRepository, Storage};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Dashboard {
    storage: Signal<Storage>,
    listing: Signal<Listing>,
}

impl Dashboard {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            listing: Signal::new(Listing::new(ConcentratesRepository::new(storage))),
        }
    }

    pub fn listing(&self) -> Signal<Listing> {
        self.listing
    }

    pub fn search_solution(&mut self, search_query: String) {
        self.listing.write().search(search_query);
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.listing.write().paginate(page_index);
    }

    pub fn create_concentrate(&self) {
        navigator().push(Route::CreateConcentratePage {
            solution_id: String::new(),
        });
    }

    pub fn open_concentrate(&self, concentrate_id: String) {
        navigator().push(Route::EditConcentratePage { concentrate_id });
    }

    /*
    pub fn create_concentrate(&self, solution_id: String) {
        navigator().push(Route::CreateConcentratePage { solution_id });
    }
    */

    pub fn delete_concentrate(&mut self, concentrate_id: String) {
        match self.storage.read().concentrates().delete(concentrate_id) {
            Ok(_) => self.listing.write().refresh(),
            Err(_) => println!("error"),
        }
    }
}
