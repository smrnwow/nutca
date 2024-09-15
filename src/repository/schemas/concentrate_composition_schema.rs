use crate::model::concentrates::Composition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ConcentrateCompositionSchema {
    FromFertilizers(HashMap<String, HashMap<String, f64>>),
    FromSolution(String, HashMap<String, HashMap<String, usize>>),
}

impl ConcentrateCompositionSchema {
    pub fn fertilizers_ids(&self) -> Vec<&String> {
        match self {
            Self::FromFertilizers(distribution) => {
                let mut fertilizers_ids: Vec<&String> = Vec::new();

                distribution.values().for_each(|part_distribution| {
                    part_distribution.keys().for_each(|fertilizer_id| {
                        fertilizers_ids.push(fertilizer_id);
                    })
                });

                fertilizers_ids
            }

            Self::FromSolution(_, _) => Vec::new(),
        }
    }
}

impl From<Composition> for ConcentrateCompositionSchema {
    fn from(composition: Composition) -> Self {
        match composition {
            Composition::FromFertilizers(composition) => {
                let mut distribution = HashMap::new();

                composition
                    .distribution()
                    .iter()
                    .for_each(|(part_id, part_distribution)| {
                        let mut part: HashMap<String, f64> = HashMap::new();

                        part_distribution.values().for_each(|fertilizer| {
                            part.insert(fertilizer.id(), fertilizer.weight());
                        });

                        distribution.insert(part_id.clone(), part);
                    });

                ConcentrateCompositionSchema::FromFertilizers(distribution)
            }

            Composition::FromSolution(composition) => ConcentrateCompositionSchema::FromSolution(
                composition.solution().id().clone(),
                composition.distribution().clone(),
            ),
        }
    }
}

impl Default for ConcentrateCompositionSchema {
    fn default() -> Self {
        Self::FromFertilizers(HashMap::new())
    }
}
