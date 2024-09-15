use super::CompositionType;
use crate::model::concentrates::{
    Composition, CompositionFromFertilizers, CompositionFromSolution, Concentrate,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ConcentrateComposition {
    composition_type: CompositionType,
    from_fertilizers: CompositionFromFertilizers,
    from_solution: CompositionFromSolution,
}

impl ConcentrateComposition {
    pub fn init(concentrate: &Concentrate) -> Self {
        match concentrate.composition() {
            Composition::FromFertilizers(composition) => Self {
                composition_type: CompositionType::FromFertilizers,
                from_fertilizers: composition.clone(),
                from_solution: CompositionFromSolution::default(),
            },

            Composition::FromSolution(composition) => Self {
                composition_type: CompositionType::FromSolution,
                from_fertilizers: CompositionFromFertilizers::default(),
                from_solution: composition.clone(),
            },
        }
    }

    pub fn exchange_composition(&mut self, composition: &Composition) -> Composition {
        match composition {
            Composition::FromFertilizers(composition) => {
                self.from_fertilizers = composition.clone();

                Composition::FromSolution(self.from_solution.clone())
            }

            Composition::FromSolution(composition) => {
                self.from_solution = composition.clone();

                Composition::FromFertilizers(self.from_fertilizers.clone())
            }
        }
    }

    pub fn change_composition_type(&mut self, composition_type: CompositionType) {
        self.composition_type = composition_type;
    }

    pub fn composition_type(&self) -> CompositionType {
        self.composition_type
    }
}
