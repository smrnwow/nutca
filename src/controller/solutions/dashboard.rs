use crate::controller::solutions::SolutionsListing;
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Dashboard {
    storage: Signal<Storage>,
    listing: Signal<SolutionsListing>,
}

impl Dashboard {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            listing: Signal::new(SolutionsListing::new(storage)),
        }
    }

    pub fn listing(&self) -> Signal<SolutionsListing> {
        self.listing
    }

    pub fn search_solution(&mut self, search_query: String) {
        self.listing.write().search(search_query);
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.listing.write().paginate(page_index);
    }

    pub fn add_solution(&self) {
        navigator().push(Route::SolutionAddPage {
            profile_id: String::new(),
        });
    }

    pub fn open_solution(&self, solution_id: String) {
        navigator().push(Route::SolutionEditPage { solution_id });
    }

    pub fn create_concentrate(&self, solution_id: String) {
        navigator().push(Route::CreateConcentratePage { solution_id });
    }

    pub fn delete_solution(&mut self, solution_id: String) {
        match self.storage.read().solutions().delete(solution_id) {
            Ok(_) => self.listing.write().refresh(),
            Err(_) => println!("error"),
        }
    }
}
