use super::{SolutionsBrowser, TanksSet};
use crate::model::chemistry::Volume;
use crate::model::concentrates::fillers::{Filler, FillerVariant};
use crate::model::concentrates::{Concentrate, StockSolutionBuilder};
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Editor {
    storage: Signal<Storage>,
    solutions_browser: SolutionsBrowser,
    tanks_set: TanksSet,
    id: Option<String>,
    name: String,
}

impl Editor {
    pub fn from_concentrate(storage: Signal<Storage>, concentrate_id: String) -> Self {
        let concentrate = match storage.read().concentrates().get(&concentrate_id) {
            Ok(concentrate) => concentrate,
            Err(_) => Concentrate::default(),
        };

        let mut solutions_browser = SolutionsBrowser::new(storage);

        if let Filler::Auto(auto_filler) = concentrate.filler() {
            solutions_browser.select(auto_filler.solution_id());
        }

        let tanks_set =
            TanksSet::from_concentrate(storage, &concentrate, solutions_browser.picked_solution());

        Self {
            storage,
            solutions_browser,
            tanks_set,
            id: Some(concentrate.id().clone()),
            name: concentrate.name().clone(),
        }
    }

    pub fn from_solution(storage: Signal<Storage>, solution_id: String) -> Self {
        let mut solutions_browser = SolutionsBrowser::new(storage);

        solutions_browser.select(&solution_id);

        let tanks_set = TanksSet::from_solution(storage, solutions_browser.picked_solution());

        Self {
            storage,
            solutions_browser,
            tanks_set,
            id: None,
            name: String::new(),
        }
    }

    pub fn solutions_browser(&self) -> &SolutionsBrowser {
        &self.solutions_browser
    }

    pub fn tanks_set(&self) -> &TanksSet {
        &self.tanks_set
    }

    pub fn concentrate(&self) -> Concentrate {
        StockSolutionBuilder::new()
            .with_id(self.id.clone())
            .with_name(self.name.clone())
            .with_filler(self.tanks_set.composition())
            .build()
    }

    pub fn search_solution(&mut self, search_query: String) {
        self.solutions_browser.search(search_query);
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn change_filler_variant(&mut self, filler_variant: FillerVariant) {
        self.tanks_set.change_filler_variant(filler_variant);
    }

    pub fn change_solution(&mut self, solution_id: String) {
        self.solutions_browser.select(&solution_id);

        self.tanks_set
            .change_solution(self.solutions_browser.picked_solution());
    }

    pub fn add_part(&mut self) {
        self.tanks_set.add_part();
    }

    pub fn delete_part(&mut self, part_id: String) {
        self.tanks_set.delete_part(part_id);
    }

    pub fn update_part_name(&mut self, part_id: String, name: String) {
        self.tanks_set.update_part_name(part_id, name);
    }

    pub fn update_part_concentration(&mut self, part_id: String, concentration: usize) {
        self.tanks_set
            .update_part_concentration(part_id, concentration);
    }

    pub fn update_part_volume(&mut self, part_id: String, volume: Volume) {
        self.tanks_set.update_part_volume(part_id, volume);
    }

    pub fn add_part_fertilizer(&mut self, part_id: String, fertilizer_id: String, value: f64) {
        self.tanks_set
            .add_part_fertilizer(part_id, fertilizer_id, value);
    }

    pub fn delete_part_fertilizer(&mut self, part_id: String, fertilizer_id: String) {
        self.tanks_set
            .delete_part_fertilizer(part_id, fertilizer_id);
    }

    pub fn create_concentrate(&self) {
        let result = self.storage.read().concentrates().add(self.concentrate());

        match result {
            Ok(_) => {
                navigator().push(Route::MainConcentratesPage {});
            }
            Err(error) => {
                println!("error {:#?}", error);
            }
        }
    }

    pub fn edit_concentrate(&self) {
        let result = self
            .storage
            .read()
            .concentrates()
            .update(self.concentrate());

        match result {
            Ok(_) => {
                navigator().push(Route::MainConcentratesPage {});
            }
            Err(error) => {
                println!("error {:#?}", error);
            }
        }
    }
}
