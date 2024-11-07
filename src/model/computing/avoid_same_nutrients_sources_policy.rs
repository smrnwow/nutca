use crate::model::chemistry::Nutrients;

pub struct AvoidSameNutrientsSourcesPolicy {
    nutrients_sources: Vec<String>,
}

impl AvoidSameNutrientsSourcesPolicy {
    pub fn new() -> Self {
        Self {
            nutrients_sources: Vec::new(),
        }
    }

    pub fn check(&mut self, nutrients_content: &Nutrients) -> bool {
        let nutrients_string = nutrients_content.stringify();

        match self.nutrients_sources.contains(&nutrients_string) {
            true => false,

            false => {
                self.nutrients_sources.push(nutrients_string);

                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AvoidSameNutrientsSourcesPolicy, Nutrients};
    use crate::model::chemistry::NutrientAmount;

    #[test]
    fn least_of_same_nutrients_sources_is_rejected() {
        let mut policy = AvoidSameNutrientsSourcesPolicy::new();

        assert_eq!(
            true,
            policy.check(&Nutrients::from(vec![NutrientAmount::Nitrogen(10.0)]))
        );

        assert_eq!(
            false,
            policy.check(&Nutrients::from(vec![NutrientAmount::Nitrogen(9.0)]))
        );
    }
}
