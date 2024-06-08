use crate::model::calculation::{Calculation, Profile, ResultProfile};
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::fertilizers::Fertilizer;
use dioxus::prelude::*;

pub struct Editor {
    pub profile: Signal<Profile>,
    pub fertilizers_composition: Signal<Vec<Fertilizer>>,
    pub result_profile: Signal<ResultProfile>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            profile: Signal::new(Profile::new()),
            fertilizers_composition: Signal::new(Vec::new()),
            result_profile: Signal::new(ResultProfile::new()),
        }
    }

    pub fn update_nutrient(&mut self, nutrient_amount: NutrientAmount) {
        self.profile.write().set_nutrient(nutrient_amount);

        *self.result_profile.write() = self.calculate();
    }

    pub fn update_nitrogen_form(&mut self, nitrogen_form: NitrogenForm) {
        self.profile.write().set_nitrogen_form(nitrogen_form);

        *self.result_profile.write() = self.calculate();
    }

    fn calculate(&self) -> ResultProfile {
        let fertilizers: Vec<Fertilizer> = self
            .fertilizers_composition
            .read()
            .iter()
            .cloned()
            .collect();

        if fertilizers.len() > 0 {
            if let Ok(result) = Calculation::new(*self.profile.read(), fertilizers.clone())
                .unwrap()
                .solve(1)
            {
                return result;
            } else {
                return ResultProfile::empty(fertilizers);
            }
        } else {
            return ResultProfile::empty(fertilizers);
        }
    }
}
