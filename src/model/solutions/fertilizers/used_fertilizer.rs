use crate::model::chemistry::Nutrients;

#[derive(Clone)]
pub struct UsedFertilizer {
    pub(super) id: isize,
    pub(super) nutrients: Nutrients,
}

impl UsedFertilizer {
    pub fn id(&self) -> isize {
        self.id
    }

    pub fn nutrients(&self) -> &Nutrients {
        &self.nutrients
    }
}

#[cfg(test)]
mod tests {
    use crate::model::chemistry::{Nutrient, NutrientAmount};
    use crate::model::solutions::fertilizers::UsedFertilizerBuilder;

    #[test]
    fn display_correct_nutrients() {
        let fertilizer = UsedFertilizerBuilder::new()
            .nutrient(NutrientAmount::Phosphorus(10.0))
            .nutrient(NutrientAmount::Potassium(25.0))
            .build();

        assert_eq!(
            10.0,
            fertilizer
                .nutrients()
                .value_of(Nutrient::Phosphorus)
                .value()
        );

        assert_eq!(
            25.0,
            fertilizer.nutrients().value_of(Nutrient::Potassium).value()
        );
    }
}
