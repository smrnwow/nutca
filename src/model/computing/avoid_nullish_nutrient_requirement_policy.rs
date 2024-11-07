use crate::model::chemistry::{Nutrient, Nutrients};

pub struct AvoidNullishNutrientRequirementPolicy<'a> {
    required_nutrition: &'a Nutrients,
}

impl<'a> AvoidNullishNutrientRequirementPolicy<'a> {
    pub fn new(required_nutrition: &'a Nutrients) -> Self {
        Self { required_nutrition }
    }

    pub fn check(&self, fertilizer_nutrients_content: &Nutrients) -> bool {
        let nutrients: Vec<Nutrient> = fertilizer_nutrients_content
            .list()
            .iter()
            .map(|nutrient_amount| nutrient_amount.nutrient())
            .collect();

        let mut has_nullish_requirement = false;

        if nutrients.len() < 4 {
            nutrients.iter().for_each(|nutrient| {
                let nutrient_requirement = self.required_nutrition.value_of(*nutrient);

                if nutrient_requirement.value() == 0.0 {
                    has_nullish_requirement = true;
                }
            });
        }

        !has_nullish_requirement
    }
}

#[cfg(test)]
mod tests {
    use super::{AvoidNullishNutrientRequirementPolicy, Nutrients};
    use crate::model::chemistry::NutrientAmount;

    #[test]
    fn fertilizer_contained_non_required_nutrient_is_rejected() {
        let required_nutrition = Nutrients::from(vec![
            NutrientAmount::Phosphorus(2.0),
            NutrientAmount::Potassium(3.0),
        ]);

        let policy = AvoidNullishNutrientRequirementPolicy::new(&required_nutrition);

        assert_eq!(
            false,
            policy.check(&Nutrients::from(vec![NutrientAmount::Nitrogen(1.5)]))
        );

        assert_eq!(
            false,
            policy.check(&Nutrients::from(vec![
                NutrientAmount::Nitrogen(1.5),
                NutrientAmount::Phosphorus(1.0)
            ]))
        );
    }

    #[test]
    fn fertilizer_contained_required_nutrient_is_accepted() {
        let required_nutrition = Nutrients::from(vec![
            NutrientAmount::Phosphorus(2.0),
            NutrientAmount::Potassium(3.0),
        ]);

        let policy = AvoidNullishNutrientRequirementPolicy::new(&required_nutrition);

        assert_eq!(
            true,
            policy.check(&Nutrients::from(vec![
                NutrientAmount::Phosphorus(1.0),
                NutrientAmount::Potassium(1.5)
            ]))
        );

        assert_eq!(
            true,
            policy.check(&Nutrients::from(vec![NutrientAmount::Potassium(1.5),]))
        );
    }
}
