use super::{CompositionFromFertilizers, CompositionFromSolution};
use crate::model::fertilizers::FertilizerAmount;

#[derive(Clone, Debug, PartialEq)]
pub enum Composition {
    FromFertilizers(CompositionFromFertilizers),
    FromSolution(CompositionFromSolution),
}

impl Composition {
    pub fn update_fertilizer_amount(&mut self, part_id: &String, fertilizer: FertilizerAmount) {
        if let Self::FromFertilizers(composition) = self {
            composition.update_fertilizer_amount(part_id, fertilizer);
        }
    }

    pub fn update_fertilizer_percent(
        &mut self,
        part_id: &String,
        fertilizer_id: &String,
        percent: usize,
    ) {
        if let Self::FromSolution(composition) = self {
            composition.update_fertilizer_percent(part_id, fertilizer_id, percent);
        }
    }

    pub fn remove_fertilizer(&mut self, part_id: &String, fertilizer_id: &String) {
        match self {
            Self::FromFertilizers(composition) => {
                composition.remove_fertilizer(part_id, fertilizer_id);
            }
            Self::FromSolution(composition) => {
                composition.remove_fertilizer(part_id, fertilizer_id);
            }
        }
    }

    pub fn remove_part(&mut self, part_id: &String) {
        match self {
            Self::FromFertilizers(composition) => composition.remove_part(part_id),
            Self::FromSolution(composition) => composition.remove_part(part_id),
        }
    }
}

impl Default for Composition {
    fn default() -> Self {
        Self::FromFertilizers(CompositionFromFertilizers::default())
    }
}
