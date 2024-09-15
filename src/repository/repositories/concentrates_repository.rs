use crate::model::concentrates::{
    Composition, CompositionFromFertilizers, CompositionFromSolution, Concentrate,
    ConcentrateSummary,
};
use crate::model::fertilizers::FertilizerAmount;
use crate::repository::schemas::{ConcentrateCompositionSchema, ConcentrateSchema};
use crate::repository::Error;
use crate::repository::{FertilizersRepository, SolutionsRepository, Storage};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ConcentratesRepository {
    storage: Signal<Storage>,
}

impl ConcentratesRepository {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self { storage }
    }

    pub fn find_or_default(&self, concentrate_id: &String) -> Concentrate {
        let concentrate_document = match self.storage.read().concentrates().get(&concentrate_id) {
            Ok(concentrate) => concentrate,
            Err(_) => ConcentrateSchema::default(),
        };

        match &concentrate_document.composition {
            ConcentrateCompositionSchema::FromFertilizers(distribution) => {
                let fertilizers_repository = FertilizersRepository::new(self.storage);
                let fertilizers_ids = concentrate_document.composition.fertilizers_ids();
                let fertilizers = fertilizers_repository.find_by_ids(fertilizers_ids);

                let mut full_distribution: HashMap<String, HashMap<String, FertilizerAmount>> =
                    HashMap::new();

                distribution.keys().for_each(|part_id| {
                    let mut part_distribution: HashMap<String, FertilizerAmount> = HashMap::new();

                    distribution.get(part_id).unwrap().iter().for_each(
                        |(fertilizer_id, amount)| {
                            let fertilizer = fertilizers
                                .iter()
                                .find(|f| f.id() == *fertilizer_id)
                                .unwrap();

                            part_distribution.insert(
                                fertilizer_id.clone(),
                                FertilizerAmount::new(fertilizer.clone(), *amount),
                            );
                        },
                    );

                    full_distribution.insert(part_id.clone(), part_distribution);
                });

                let composition = CompositionFromFertilizers::new(full_distribution);

                Concentrate::new(
                    concentrate_document.id,
                    concentrate_document.name,
                    Composition::FromFertilizers(composition),
                    concentrate_document
                        .parts
                        .iter()
                        .cloned()
                        .map(|part| part.into())
                        .collect(),
                )
            }

            ConcentrateCompositionSchema::FromSolution(solution_id, distribution) => {
                let solutions_repository = SolutionsRepository::new(self.storage);
                let solution = solutions_repository.get(&solution_id);
                let composition = CompositionFromSolution::restore(solution, distribution.clone());

                Concentrate::new(
                    concentrate_document.id,
                    concentrate_document.name,
                    Composition::FromSolution(composition),
                    concentrate_document
                        .parts
                        .iter()
                        .cloned()
                        .map(|part| part.into())
                        .collect(),
                )
            }
        }
    }

    pub fn create(&self, concentrate: Concentrate) -> Result<(), Error> {
        let concentrate_document = ConcentrateSchema::from(concentrate);

        self.storage.read().concentrates().add(concentrate_document)
    }

    pub fn update(&self, concentrate: Concentrate) -> Result<(), Error> {
        let concentrate_document = ConcentrateSchema::from(concentrate);

        self.storage
            .read()
            .concentrates()
            .update(concentrate_document)
    }

    pub fn search(
        &self,
        query: &String,
        limit: usize,
        page_index: usize,
    ) -> Vec<ConcentrateSummary> {
        match self
            .storage
            .read()
            .concentrates()
            .search(query, limit, limit * (page_index - 1))
        {
            Ok(concentrates) => concentrates
                .into_iter()
                .map(|concentrate_document| concentrate_document.into())
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}
