use super::UsedFertilizer;
use crate::model::chemistry::{Nutrients, Volume};

#[derive(Clone)]
pub struct FertilizerAmount {
    fertilizer: UsedFertilizer,
    amount: f64,
}

impl FertilizerAmount {
    pub fn new(fertilizer: UsedFertilizer, amount: f64) -> Self {
        Self { fertilizer, amount }
    }

    pub fn by_volume(self, volume: &Volume) -> Self {
        Self {
            fertilizer: self.fertilizer,
            amount: self.amount * volume.to_litres(),
        }
    }

    pub fn id(&self) -> isize {
        self.fertilizer.id()
    }

    pub fn nutrients(&self) -> Nutrients {
        self.fertilizer.nutrients().multiply(self.amount)
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

#[cfg(test)]
mod tests {
    use super::FertilizerAmount;
    use crate::model::chemistry::{Nutrient, NutrientAmount};
    use crate::model::solutions::fertilizers::UsedFertilizerBuilder;

    #[test]
    fn amount_applied_to_nutrients_as_coefficient() {
        let fertilizer = UsedFertilizerBuilder::new()
            .nutrient(NutrientAmount::NitrogenNitrate(10.0))
            .nutrient(NutrientAmount::Phosphorus(5.0))
            .nutrient(NutrientAmount::Potassium(25.0))
            .build();

        let fertilizer_amount = FertilizerAmount::new(fertilizer, 1.5);

        assert_eq!(
            15.0,
            fertilizer_amount
                .nutrients()
                .value_of(Nutrient::NitrogenNitrate)
                .value()
        );

        assert_eq!(
            7.5,
            fertilizer_amount
                .nutrients()
                .value_of(Nutrient::Phosphorus)
                .value()
        );

        assert_eq!(
            37.5,
            fertilizer_amount
                .nutrients()
                .value_of(Nutrient::Potassium)
                .value()
        );
    }
}
