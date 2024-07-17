use crate::repository::{SolutionsListing, Storage};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Dashboard {
    storage: Signal<Storage>,
    listing: SolutionsListing,
}

impl Dashboard {
    pub fn new(storage: Signal<Storage>) -> Self {
        let listing = match storage.read().solutions().list() {
            Ok(listing) => listing,
            Err(_) => SolutionsListing::new(vec![]),
        };

        Self { storage, listing }
    }

    pub fn list_solutions(&self) -> SolutionsListing {
        self.listing.clone()
    }

    pub fn search_solution(&mut self, search_query: String) {
        self.listing.search(search_query);
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.listing.paginate(page_index);
    }

    pub fn add_solution(&self) {
        navigator().push(Route::SolutionAddPage {
            profile_id: String::new(),
        });
    }

    pub fn open_solution(&self, solution_id: String) {
        navigator().push(Route::SolutionEditPage { solution_id });
    }

    pub fn open_stock_solution(&self, solution_id: String) {
        navigator().push(Route::StockSolutionPage { solution_id });
    }

    pub fn delete_solution(&mut self, solution_id: String) {
        match self.storage.read().solutions().delete(solution_id) {
            Ok(_) => {
                self.listing = match self.storage.read().solutions().list() {
                    Ok(listing) => listing,
                    Err(_) => SolutionsListing::new(vec![]),
                };
            }

            Err(_) => println!("error"),
        }
    }
}
