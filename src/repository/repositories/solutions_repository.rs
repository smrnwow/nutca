use super::{FertilizersRepository, NutritionProgramsRepository};
use crate::model::chemistry::Nutrients;
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizerWeight, NutrientComposition, Solution, SolutionSummary};
use crate::repository::schemas::{NutritionProgramSchema, SolutionSchema};
use crate::repository::Error;
use crate::repository::Storage;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionsRepository {
    storage: Signal<Storage>,
}

impl SolutionsRepository {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self { storage }
    }

    pub fn get(&self, solution_id: &String) -> Solution {
        let nutrition_programs_repository = NutritionProgramsRepository::new(self.storage);

        let fertilizers_repository = FertilizersRepository::new(self.storage);

        let solution_document = match self.storage.read().solutions().get(&solution_id) {
            Ok(solution_document) => solution_document,
            Err(_) => SolutionSchema::default(),
        };

        let nutrition_program = match solution_document.nutrition_program {
            NutritionProgramSchema::Saved(nutrition_program_id) => {
                nutrition_programs_repository.find_or_default(&nutrition_program_id)
            }

            NutritionProgramSchema::ByValue(nutrients) => Profile::from(nutrients),
        };

        let mut nutrient_composition =
            NutrientComposition::new(nutrition_program, Nutrients::new());

        let fertilizers =
            fertilizers_repository.find_by_ids(solution_document.fertilizers.keys().collect());

        let fertilizers = fertilizers
            .into_iter()
            .fold(HashMap::new(), |mut acc, fertilizer| {
                if let Some(fertilizer_amount) = solution_document.fertilizers.get(&fertilizer.id())
                {
                    acc.insert(
                        fertilizer.id(),
                        FertilizerWeight::new(fertilizer, *fertilizer_amount),
                    );
                }

                acc
            });

        nutrient_composition.with_fertilizers_amounts(fertilizers.values().collect());

        Solution::new(
            solution_document.id,
            solution_document.name,
            nutrient_composition,
            fertilizers,
            solution_document.volume,
        )
    }

    pub fn create(&self, solution: Solution) -> Result<(), Error> {
        let solution_document = SolutionSchema::from(solution);

        match self.storage.read().solutions().add(solution_document) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn update(&self, solution: Solution) -> Result<(), Error> {
        let solution_document = SolutionSchema::from(solution);

        match self.storage.read().solutions().update(solution_document) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn search(&self, query: &String, limit: usize, page_index: usize) -> Vec<SolutionSummary> {
        match self
            .storage
            .read()
            .solutions()
            .search(query, limit, limit * (page_index - 1))
        {
            Ok(solutions) => solutions
                .into_iter()
                .map(|solution| solution.into())
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}
