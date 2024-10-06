use super::Stage;
use crate::model::chemistry::{NutrientAmount, Nutrients};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Profile {
    id: String,
    name: String,
    stages: Vec<Stage>,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            stages: Vec::from([Stage::from(Nutrients::new())]),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn stages(&self) -> &Vec<Stage> {
        &self.stages
    }

    pub fn first_stage(&self) -> &Stage {
        self.stages.get(0).unwrap()
    }

    pub fn stage(&self, stage_id: &str) -> Option<&Stage> {
        let position = self
            .stages
            .iter()
            .position(|stage| *stage.id() == *stage_id);

        match position {
            Some(index) => self.stages.get(index),
            None => None,
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_stage(&mut self, new_stage: Stage) {
        self.stages.push(new_stage);
    }

    pub fn update_stage_name(&mut self, stage_id: &str, name: String) {
        let position = self
            .stages
            .iter()
            .position(|stage| *stage.id() == *stage_id);

        if let Some(index) = position {
            if let Some(stage) = self.stages.get_mut(index) {
                stage.update_name(name);
            }
        }
    }

    pub fn update_nutrient(&mut self, stage_id: &str, nutrient_amount: NutrientAmount) {
        let position = self
            .stages
            .iter()
            .position(|stage| *stage.id() == *stage_id);

        if let Some(index) = position {
            if let Some(stage) = self.stages.get_mut(index) {
                stage.update_nutrient(nutrient_amount);
            }
        }
    }

    pub fn remove_stage(&mut self, stage_id: &str) {
        if self.stages.len() > 1 {
            let position = self
                .stages
                .iter()
                .position(|stage| *stage.id() == *stage_id);

            if let Some(index) = position {
                self.stages.remove(index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NutrientAmount, Profile, Stage};
    use crate::model::chemistry::Nutrient;

    #[test]
    fn new_nutrition_program_has_single_stage() {
        let nutrition_program = Profile::new();

        assert_eq!(1, nutrition_program.stages().len());
    }

    #[test]
    fn new_stage_can_be_added_to_nutrition_program() {
        let mut nutrition_program = Profile::new();

        nutrition_program.add_stage(Stage::new());

        assert_eq!(2, nutrition_program.stages().len());
    }

    #[test]
    fn stages_can_be_removed_from_nutrition_program() {
        let mut nutrition_program = Profile::new();

        let new_stage = Stage::new();

        nutrition_program.add_stage(new_stage.clone());

        nutrition_program.remove_stage(new_stage.id());

        assert_eq!(1, nutrition_program.stages().len());
    }

    #[test]
    fn last_stage_cannot_be_removed_from_nutrition_program() {
        let mut nutrition_program = Profile::new();

        let new_stage = Stage::new();

        nutrition_program.add_stage(new_stage.clone());

        let stages_ids: Vec<String> = nutrition_program
            .stages()
            .iter()
            .map(|stage| stage.id().to_string())
            .collect();

        stages_ids.iter().for_each(|stage_id| {
            nutrition_program.remove_stage(stage_id);
        });

        assert_eq!(1, nutrition_program.stages().len());
    }

    #[test]
    fn nutrition_program_stage_name_can_be_updated() {
        let mut nutrition_program = Profile::new();

        let stage = Stage::new();

        nutrition_program.add_stage(stage.clone());

        let stage_name = "stage name";

        nutrition_program.update_stage_name(stage.id(), stage_name.to_string());

        if let Some(stage) = nutrition_program.stage(stage.id()) {
            assert_eq!(stage_name, stage.name());
        }
    }

    #[test]
    fn nutrition_program_stage_nutrient_can_be_updated() {
        let mut nutrition_program = Profile::new();

        let stage = Stage::new();

        nutrition_program.add_stage(stage.clone());

        nutrition_program.update_nutrient(stage.id(), NutrientAmount::Magnesium(56.0));

        if let Some(stage) = nutrition_program.stage(stage.id()) {
            assert_eq!(56.0, stage.nutrients()[Nutrient::Magnesium].value());
        }
    }

    #[test]
    fn nutrition_program_name_can_be_changed() {
        let mut nutrition_program = Profile::new();

        let name = "nutrition program name";

        assert_eq!("", nutrition_program.name());

        nutrition_program.update_name(name.to_string());

        assert_eq!(name, nutrition_program.name());
    }

    #[test]
    fn random_id_is_generated_for_new_nutrition_program() {
        let nutrition_program = Profile::new();

        assert!(nutrition_program.id().len() > 0);
    }
}
