use super::{CompositionType, ConcentrateComposition, FertilizersBrowser, SolutionsBrowser};
use crate::model::chemistry::Volume;
use crate::model::concentrates::Concentrate;
use crate::model::solutions::FertilizerWeight;
use crate::repository::ConcentratesRepository;
use crate::ui::router::Route;
use dioxus_router::prelude::*;

pub struct Editor {
    concentrates_repository: ConcentratesRepository,
    concentrate: Concentrate,
    concentrate_composition: ConcentrateComposition,
    solutions_browser: SolutionsBrowser,
    fertilizers_browser: FertilizersBrowser,
}

impl Editor {
    pub fn new(
        concentrates_repository: ConcentratesRepository,
        concentrate: Concentrate,
        solutions_browser: SolutionsBrowser,
        fertilizers_browser: FertilizersBrowser,
    ) -> Self {
        let concentrate_composition = ConcentrateComposition::init(&concentrate);

        Self {
            concentrates_repository,
            concentrate,
            concentrate_composition,
            solutions_browser,
            fertilizers_browser,
        }
    }

    pub fn concentrate(&self) -> &Concentrate {
        &self.concentrate
    }

    pub fn concentrate_composition(&self) -> &ConcentrateComposition {
        &self.concentrate_composition
    }

    pub fn solutions_browser(&self) -> &SolutionsBrowser {
        &self.solutions_browser
    }

    pub fn fertilizers_browser(&self) -> &FertilizersBrowser {
        &self.fertilizers_browser
    }

    pub fn search_solution(&mut self, search_query: String) {
        self.solutions_browser.search(search_query);
    }

    pub fn update_name(&mut self, name: String) {
        self.concentrate.update_name(name);
    }

    pub fn change_composition_type(&mut self, composition_type: CompositionType) {
        let composition = self
            .concentrate_composition
            .exchange_composition(self.concentrate.composition());

        self.concentrate.set_composition(composition);

        self.concentrate_composition
            .change_composition_type(composition_type);
    }

    pub fn change_solution(&mut self, solution_id: String) {
        let solution = self.solutions_browser.select(&solution_id);

        self.concentrate.change_solution(solution);
    }

    pub fn add_part(&mut self) {
        self.concentrate.add_part();
    }

    pub fn delete_part(&mut self, part_id: String) {
        self.concentrate.remove_part(&part_id);
    }

    pub fn update_part_name(&mut self, part_id: String, name: String) {
        if let Some(part) = self.concentrate.get_part(&part_id) {
            part.update_name(name);
        }
    }

    pub fn update_part_concentration(&mut self, part_id: String, concentration: usize) {
        if let Some(part) = self.concentrate.get_part(&part_id) {
            part.update_concentration(concentration);
        }
    }

    pub fn update_part_volume(&mut self, part_id: String, volume: Volume) {
        if let Some(part) = self.concentrate.get_part(&part_id) {
            part.update_volume(volume);
        }
    }

    pub fn update_fertilizer_percent(
        &mut self,
        part_id: String,
        fertilizer_id: String,
        percent: usize,
    ) {
        self.concentrate
            .composition_mut()
            .update_fertilizer_percent(&part_id, &fertilizer_id, percent);
    }

    pub fn update_fertilizer_amount(
        &mut self,
        part_id: String,
        fertilizer_id: String,
        amount: f64,
    ) {
        if let Some(fertilizer) = self.fertilizers_browser.get(&fertilizer_id) {
            self.concentrate
                .composition_mut()
                .update_fertilizer_amount(&part_id, FertilizerWeight::new(fertilizer, amount));
        }
    }

    pub fn remove_fertilizer(&mut self, part_id: String, fertilizer_id: String) {
        self.concentrate
            .composition_mut()
            .remove_fertilizer(&part_id, &fertilizer_id);
    }

    pub fn create_concentrate(&self) {
        match self
            .concentrates_repository
            .create(self.concentrate.clone())
        {
            Ok(_) => {
                navigator().push(Route::MainConcentratesPage {});
            }
            Err(error) => {
                println!("error {:#?}", error);
            }
        }
    }

    pub fn edit_concentrate(&self) {
        match self
            .concentrates_repository
            .update(self.concentrate.clone())
        {
            Ok(_) => {
                navigator().push(Route::MainConcentratesPage {});
            }
            Err(error) => {
                println!("error {:#?}", error);
            }
        }
    }
}
