use super::{FertilizersBrowser, FertilizersStack};
use crate::controller::solutions::SolutionsListing;
use crate::model::chemistry::Volume;
use crate::model::concentrates::fillers::{AutoFiller, Filler, FillerVariant, ManualFiller};
use crate::model::concentrates::{Concentrate, StockSolutionBuilder};
use crate::model::solutions::Solution;
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct Editor {
    storage: Signal<Storage>,
    solution: Solution,
    solutions_listing: SolutionsListing,
    fertilizers_stack: FertilizersStack,
    fertilizers_browser: FertilizersBrowser,
    id: Option<String>,
    name: String,
    filler_variant: FillerVariant,
    auto_filler: AutoFiller,
    manual_filler: ManualFiller,
}

impl Editor {
    pub fn from_concentrate(storage: Signal<Storage>, concentrate_id: String) -> Self {
        let concentrate = match storage.read().concentrates().get(&concentrate_id) {
            Ok(concentrate) => concentrate,
            Err(_) => Concentrate::default(),
        };

        let solution = match concentrate.filler() {
            Filler::Auto(auto_filler) => {
                match storage.read().solutions().get(auto_filler.solution_id()) {
                    Ok(solution) => solution,
                    Err(_) => Solution::default(),
                }
            }
            Filler::Manual(_) => Solution::default(),
        };

        let auto_filler = match concentrate.filler() {
            Filler::Auto(auto_filler) => auto_filler.clone(),
            Filler::Manual(_) => AutoFiller::new(&solution),
        };

        let manual_filler = match concentrate.filler() {
            Filler::Auto(_) => ManualFiller::new(),
            Filler::Manual(manual_filler) => manual_filler.clone(),
        };

        let filler_variant = match concentrate.filler() {
            Filler::Auto(_) => FillerVariant::Auto,
            Filler::Manual(_) => FillerVariant::Manual,
        };

        let mut fertilizers_stack = FertilizersStack::new(storage);

        fertilizers_stack
            .with_amounts(solution.fertilizers())
            .with_stack(auto_filler.stack());

        Self {
            storage,
            solution,
            solutions_listing: SolutionsListing::new(storage),
            fertilizers_stack,
            fertilizers_browser: FertilizersBrowser::new(storage),
            id: Some(concentrate.id().clone()),
            name: concentrate.name().clone(),
            filler_variant,
            auto_filler,
            manual_filler,
        }
    }

    pub fn from_solution(storage: Signal<Storage>, solution_id: String) -> Self {
        let solution = match storage.read().solutions().get(&solution_id) {
            Ok(solution) => solution,
            Err(_) => Solution::default(),
        };

        let auto_filler = AutoFiller::new(&solution);

        let mut fertilizers_stack = FertilizersStack::new(storage);

        fertilizers_stack
            .with_amounts(solution.fertilizers())
            .with_stack(auto_filler.stack());

        Self {
            storage,
            solution,
            solutions_listing: SolutionsListing::new(storage),
            fertilizers_stack,
            fertilizers_browser: FertilizersBrowser::new(storage),
            id: None,
            name: String::new(),
            filler_variant: FillerVariant::Auto,
            auto_filler,
            manual_filler: ManualFiller::new(),
        }
    }

    pub fn list_solutions(&self) -> SolutionsListing {
        self.solutions_listing.clone()
    }

    pub fn fertilizers_stack(&self) -> &FertilizersStack {
        &self.fertilizers_stack
    }

    pub fn fertilizers_browser(&self) -> &FertilizersBrowser {
        &self.fertilizers_browser
    }

    pub fn solution(&self) -> &Solution {
        &self.solution
    }

    pub fn filler_variant(&self) -> FillerVariant {
        self.filler_variant
    }

    pub fn auto_filler(&self) -> &AutoFiller {
        &self.auto_filler
    }

    pub fn manual_filler(&self) -> &ManualFiller {
        &self.manual_filler
    }

    pub fn concentrate(&self) -> Concentrate {
        let mut builder = StockSolutionBuilder::new();

        builder
            .with_id(self.id.clone())
            .with_name(self.name.clone());

        match self.filler_variant {
            FillerVariant::Auto => builder.with_auto_filler(self.auto_filler.clone()),
            FillerVariant::Manual => builder.with_manual_filler(self.manual_filler.clone()),
        };

        builder.build()
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn change_filler_variant(&mut self, filler_variant: FillerVariant) {
        self.filler_variant = filler_variant;
    }

    pub fn search_solution(&mut self, search_query: String) {
        self.solutions_listing.search(search_query);
    }

    pub fn change_solution(&mut self, solution_id: String) {
        if let Some(solution) = self.solutions_listing.find(solution_id) {
            self.auto_filler = AutoFiller::new(&solution);

            self.solution = solution;

            self.fertilizers_stack
                .with_amounts(self.solution.fertilizers())
                .with_stack(self.auto_filler.stack());
        }
    }

    pub fn add_part(&mut self) {
        match self.filler_variant {
            FillerVariant::Auto => self.auto_filler.add_part(),
            FillerVariant::Manual => self.manual_filler.add_part(),
        };
    }

    pub fn delete_part(&mut self, part_id: String) {
        match self.filler_variant {
            FillerVariant::Auto => {
                self.auto_filler.delete_part(part_id);

                self.fertilizers_stack.with_stack(self.auto_filler.stack());
            }
            FillerVariant::Manual => self.manual_filler.delete_part(part_id),
        };
    }

    pub fn update_part_name(&mut self, part_id: String, name: String) -> Option<()> {
        match self.filler_variant {
            FillerVariant::Auto => {
                self.auto_filler.get_part(&part_id)?.update_name(name);
                Some(())
            }

            FillerVariant::Manual => self
                .manual_filler
                .get_part(&part_id)
                .and_then(|part| Some(part.update_name(name))),
        }
    }

    pub fn update_part_concentration(&mut self, part_id: String, concentration: usize) {
        match self.filler_variant {
            FillerVariant::Auto => self
                .auto_filler
                .update_part_concentration(part_id, concentration),
            FillerVariant::Manual => self
                .manual_filler
                .update_part_concentration(part_id, concentration),
        };
    }

    pub fn update_part_volume(&mut self, part_id: String, volume: Volume) {
        match self.filler_variant {
            FillerVariant::Auto => self.auto_filler.update_part_volume(part_id, volume),
            FillerVariant::Manual => self.manual_filler.update_part_volume(part_id, volume),
        };
    }

    pub fn add_part_fertilizer(&mut self, part_id: String, fertilizer_id: String, value: f64) {
        match self.filler_variant {
            FillerVariant::Auto => {
                let fertilizer_weight = self.solution.get_fertilizer_amount(&fertilizer_id);

                if let Some(fertilizer_weight) = fertilizer_weight {
                    self.auto_filler.add_fertilizer(
                        part_id,
                        fertilizer_weight.id(),
                        fertilizer_weight.weight(),
                        value as usize,
                    );

                    self.fertilizers_stack.with_stack(self.auto_filler.stack());
                }
            }

            FillerVariant::Manual => {
                let fertilizer = self.fertilizers_browser.get(&fertilizer_id);

                if let Some(fertilizer) = fertilizer {
                    self.manual_filler
                        .add_part_fertilizer(part_id, fertilizer, value);
                }
            }
        };
    }

    pub fn delete_part_fertilizer(&mut self, part_id: String, fertilizer_id: String) {
        match self.filler_variant {
            FillerVariant::Auto => {
                self.auto_filler.delete_fertilizer(part_id, fertilizer_id);

                self.fertilizers_stack.with_stack(self.auto_filler.stack());
            }
            FillerVariant::Manual => self
                .manual_filler
                .delete_part_fertilizer(part_id, fertilizer_id),
        };
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
