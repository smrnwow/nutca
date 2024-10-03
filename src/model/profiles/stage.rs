use crate::model::chemistry::{NutrientAmount, Nutrients};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Stage {
    id: String,
    name: String,
    nutrients: Nutrients,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            nutrients: Nutrients::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn nutrients(&self) -> &Nutrients {
        &self.nutrients
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_nutrient(&mut self, nutrient_amount: NutrientAmount) {
        self.nutrients.set(nutrient_amount);
    }
}

impl From<Nutrients> for Stage {
    fn from(nutrients: Nutrients) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            nutrients,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NutrientAmount, Nutrients, Stage};
    use crate::model::chemistry::Nutrient;

    #[test]
    fn stage_created_from_nutrients_contains_these_nutrients() {
        let nutrients = Nutrients::new();

        assert_eq!(&nutrients, Stage::from(nutrients).nutrients())
    }

    #[test]
    fn stage_name_can_be_updated() {
        let mut stage = Stage::new();

        assert_eq!("", stage.name());

        let stage_name = "stage name";

        stage.update_name(stage_name.to_string());

        assert_eq!(*stage_name, *stage.name());
    }

    #[test]
    fn stage_nutrient_can_be_updated() {
        let mut stage = Stage::new();

        assert_eq!(0.0, stage.nutrients()[Nutrient::Nitrogen].value());

        stage.update_nutrient(NutrientAmount::Nitrogen(10.0));

        assert_eq!(10.0, stage.nutrients()[Nutrient::Nitrogen].value());
    }
}
