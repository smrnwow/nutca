use super::{Editor, FertilizersPicker, NutritionProgramBrowser, SolutionValidator};
use crate::controller::Toaster;
use crate::model::solutions::{NutrientComposition, Solution};
use crate::repository::{
    FertilizersRepository, NutritionProgramsRepository, SolutionsRepository, Storage,
};
use dioxus::prelude::*;

pub struct EditorFactory {
    storage: Signal<Storage>,
    toaster: Signal<Toaster>,
}

impl EditorFactory {
    pub fn new(storage: Signal<Storage>, toaster: Signal<Toaster>) -> Self {
        Self { storage, toaster }
    }

    pub fn create(&self, nutrition_program_id: String) -> Editor {
        let solutions_repository = SolutionsRepository::new(self.storage);

        let nutrition_programs_repository = NutritionProgramsRepository::new(self.storage);
        let nutrition_program_browser = NutritionProgramBrowser::new(nutrition_programs_repository);
        let nutrition_program = nutrition_program_browser.find(&nutrition_program_id);

        let fertilizers_repository = FertilizersRepository::new(self.storage);
        let fertilizers_picker = FertilizersPicker::new(fertilizers_repository);

        let mut nutrient_composition = NutrientComposition::default();
        nutrient_composition.with_nutrition_program(nutrition_program);

        let solution = Solution::from(nutrient_composition);

        let mut solution_validator = SolutionValidator::new(self.toaster);
        solution_validator.init(&solution);

        Editor::new(
            solutions_repository,
            solution,
            solution_validator,
            nutrition_program_browser,
            fertilizers_picker,
        )
    }

    pub fn edit(&self, solution_id: String) -> Editor {
        let solutions_repository = SolutionsRepository::new(self.storage);
        let solution = solutions_repository.get(&solution_id);

        let nutrition_programs_repository = NutritionProgramsRepository::new(self.storage);
        let nutrition_program_browser = NutritionProgramBrowser::new(nutrition_programs_repository);

        let fertilizers_repository = FertilizersRepository::new(self.storage);
        let mut fertilizers_picker = FertilizersPicker::new(fertilizers_repository);
        fertilizers_picker.set_projection(solution.fertilizers().keys().collect());

        let mut solution_validator = SolutionValidator::new(self.toaster);
        solution_validator.init(&solution);

        Editor::new(
            solutions_repository,
            solution,
            solution_validator,
            nutrition_program_browser,
            fertilizers_picker,
        )
    }
}
