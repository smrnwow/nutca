use super::{FertilizersRepository, ProfilesRepository};
use crate::model::fertilizers::FertilizerAmount;
use crate::model::solutions::{NutritionContent, ProfileRequirement, Solution, SolutionSummary};
use crate::repository::schemas::{ProfileRequirementSchema, SolutionSchema};
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
        let profiles_repository = ProfilesRepository::new(self.storage);

        let fertilizers_repository = FertilizersRepository::new(self.storage);

        let solution_document = match self.storage.read().solutions().get(&solution_id) {
            Ok(solution_document) => solution_document,
            Err(_) => SolutionSchema::default(),
        };

        let profile_requirement = match solution_document.profile_requirement {
            ProfileRequirementSchema::Saved(profile_id, stage_id) => {
                let profile = profiles_repository.find_or_default(&profile_id);

                ProfileRequirement::from((profile, stage_id))
            }

            ProfileRequirementSchema::ByValue(nutrients) => ProfileRequirement::from(nutrients),
        };

        let fertilizers =
            fertilizers_repository.find_by_ids(solution_document.fertilizers.keys().collect());

        let fertilizers = fertilizers
            .into_iter()
            .fold(HashMap::new(), |mut acc, fertilizer| {
                if let Some(fertilizer_amount) = solution_document.fertilizers.get(&fertilizer.id())
                {
                    acc.insert(
                        fertilizer.id(),
                        FertilizerAmount::new(fertilizer, *fertilizer_amount),
                    );
                }

                acc
            });

        let mut nutrition_content = NutritionContent::new();
        nutrition_content.calculate(fertilizers.values().collect());

        Solution::new(
            solution_document.id,
            solution_document.name,
            profile_requirement,
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
