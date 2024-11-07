use super::{FertilizerAmount, UsedFertilizer};
use crate::model::chemistry::Volume;
use crate::model::solutions::computing::CalculationResult;
use std::collections::HashMap;

pub struct FertilizersSet {
    fertilizers: HashMap<isize, UsedFertilizer>,
    amounts: HashMap<isize, f64>,
    volume: Volume,
}

impl FertilizersSet {
    pub fn new() -> Self {
        Self {
            fertilizers: HashMap::new(),
            amounts: HashMap::new(),
            volume: Volume::default(),
        }
    }

    pub fn list(&self) -> Vec<FertilizerAmount> {
        self.fertilizers
            .values()
            .map(|fertilizer| {
                FertilizerAmount::new(fertilizer, self.amount(fertilizer.id()))
                    .by_volume(self.volume())
            })
            .collect()
    }

    pub fn get(&self, fertilizer_id: isize) -> Option<FertilizerAmount> {
        match self.fertilizers.get(fertilizer_id) {
            Some(fertilizer) => {
                let fertilizer_amount =
                    FertilizerAmount::new(*fertilizer, self.amount(fertilizer.id()))
                        .by_volume(self.volume());

                Some(fertilizer_amount)
            }

            None => None,
        }
    }

    pub fn volume(&self) -> &Volume {
        &self.volume
    }

    pub fn add(&mut self, fertilizer: UsedFertilizer) {
        self.amounts.insert(fertilizer.id().to_string(), 0.0);

        self.fertilizers
            .insert(fertilizer.id().to_string(), fertilizer);
    }

    pub fn remove(&mut self, fertilizer_id: &str) {
        self.fertilizers.remove(fertilizer_id);
    }

    pub fn update_amount(&mut self, fertilizer_id: &str, amount: f64) {
        self.amounts.insert(fertilizer_id.to_string(), amount);
    }

    pub fn update_many(&mut self, calculation_result: CalculationResult) {
        calculation_result
            .portions()
            .into_iter()
            .for_each(|(fertilizer_id, portion)| {
                self.amounts
                    .insert(fertilizer_id.to_string(), portion.amount());
            });
    }

    pub fn change_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    fn amount(&self, fertilizer_id: &str) -> f64 {
        match self.amounts.get(fertilizer_id) {
            Some(amount) => *amount,
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Fertilizer, FertilizersSet};
    use crate::model::chemistry::{NutrientAmount, Volume, VolumeUnits};
    use crate::model::solutions::computing::{CalculationResult, FertilizerPortion};

    #[test]
    fn new_fertilizer_can_be_added_to_set() {
        let mut fertilizers_set = FertilizersSet::new();

        fertilizers_set.add(Fertilizer::from(vec![
            NutrientAmount::NitrogenNitrate(30.0),
            NutrientAmount::Phosphorus(10.0),
        ]));

        assert_eq!(1, fertilizers_set.list().len());
    }

    #[test]
    fn fertilizer_can_be_removed_from_set() {
        let mut fertilizers_set = FertilizersSet::new();

        let fertilizer = Fertilizer::from(vec![
            NutrientAmount::NitrogenNitrate(30.0),
            NutrientAmount::Phosphorus(10.0),
        ]);

        let fertilizer_id = fertilizer.id().to_string();

        fertilizers_set.add(fertilizer);

        fertilizers_set.remove(&fertilizer_id);

        assert_eq!(0, fertilizers_set.list().len());
    }

    #[test]
    fn amount_of_fertilizer_in_set_can_be_updated() {
        let mut fertilizers_set = FertilizersSet::new();

        let fertilizer = Fertilizer::from(vec![
            NutrientAmount::NitrogenNitrate(30.0),
            NutrientAmount::Phosphorus(10.0),
        ]);

        let fertilizer_id = fertilizer.id().to_string();

        fertilizers_set.add(fertilizer);

        fertilizers_set.update_amount(&fertilizer_id, 1.5);

        assert_eq!(1.5, fertilizers_set.get(&fertilizer_id).unwrap().amount());
    }

    #[test]
    fn volume_can_be_changed() {
        let mut fertilizers_set = FertilizersSet::new();

        fertilizers_set.change_volume(Volume::new(2, VolumeUnits::Litres));

        assert_eq!(2, fertilizers_set.volume().to_litres() as isize);
    }

    #[test]
    fn volume_is_applied_to_fertilizers_amounts() {
        let mut fertilizers_set = FertilizersSet::new();

        let fertilizer = Fertilizer::from(vec![
            NutrientAmount::NitrogenNitrate(30.0),
            NutrientAmount::Phosphorus(10.0),
        ]);

        let fertilizer_id = fertilizer.id().to_string();

        fertilizers_set.add(fertilizer);

        fertilizers_set.update_amount(&fertilizer_id, 2.5);

        fertilizers_set.change_volume(Volume::new(2, VolumeUnits::Litres));

        assert_eq!(5.0, fertilizers_set.get(&fertilizer_id).unwrap().amount());
    }

    #[test]
    fn calculation_result_is_applied_to_fertilizers_set() {
        let mut fertilizers_set = FertilizersSet::new();

        let fertilizer = Fertilizer::from(vec![
            NutrientAmount::NitrogenNitrate(30.0),
            NutrientAmount::Phosphorus(10.0),
        ]);

        let fertilizer_id = fertilizer.id().to_string();

        let calculation_result = CalculationResult::from(vec![(
            fertilizer_id.clone(),
            FertilizerPortion::Calculated(1.0),
        )]);

        fertilizers_set.add(fertilizer);

        fertilizers_set.update_many(calculation_result);

        assert_eq!(1.0, fertilizers_set.get(&fertilizer_id).unwrap().amount());
    }
}
