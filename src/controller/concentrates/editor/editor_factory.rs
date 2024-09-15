use super::{Editor, FertilizersBrowser, SolutionsBrowser};
use crate::model::concentrates::Concentrate;
use crate::repository::{
    ConcentratesRepository, FertilizersRepository, SolutionsRepository, Storage,
};
use dioxus::prelude::*;

pub struct EditorFactory {
    storage: Signal<Storage>,
}

impl EditorFactory {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self { storage }
    }

    pub fn create(&self, solution_id: String) -> Editor {
        let solutions_repository = SolutionsRepository::new(self.storage);
        let solution = solutions_repository.get(&solution_id);
        let solutions_browser = SolutionsBrowser::new(solutions_repository);

        let fertilizers_repository = FertilizersRepository::new(self.storage);
        let fertilizers_browser = FertilizersBrowser::new(fertilizers_repository);

        let concentrates_repository = ConcentratesRepository::new(self.storage);
        let concentrate = Concentrate::from(solution);

        Editor::new(
            concentrates_repository,
            concentrate,
            solutions_browser,
            fertilizers_browser,
        )
    }

    pub fn update(&self, concentrate_id: String) -> Editor {
        let concentrates_repository = ConcentratesRepository::new(self.storage);
        let concentrate = concentrates_repository.find_or_default(&concentrate_id);

        let solutions_repository = SolutionsRepository::new(self.storage);
        let solutions_browser = SolutionsBrowser::new(solutions_repository);

        let fertilizers_repository = FertilizersRepository::new(self.storage);
        let fertilizers_browser = FertilizersBrowser::new(fertilizers_repository);

        Editor::new(
            concentrates_repository,
            concentrate,
            solutions_browser,
            fertilizers_browser,
        )
    }
}
