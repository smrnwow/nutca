use super::{FertilizersBrowser, SelectedSet};
use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::CalculationResult;
use crate::repository::Storage;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersPicker {
    pub(crate) browser: FertilizersBrowser,
    pub(crate) selected_set: SelectedSet,
}

impl FertilizersPicker {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            browser: FertilizersBrowser::new(storage),
            selected_set: SelectedSet::new(),
        }
    }

    pub fn browse(&self) -> Vec<Fertilizer> {
        self.browser
            .fetch(self.selected_set.selected_fertilizers_ids())
    }

    pub fn with_picked_fertilizers(
        &mut self,
        fertilizers_amounts: &HashMap<String, CalculationResult>,
    ) -> &mut Self {
        let fertilizers = self.browser.filter(fertilizers_amounts.keys().collect());

        self.selected_set
            .set_selected_fertilizers(fertilizers_amounts, fertilizers);

        self
    }

    pub fn pick_fertilizer(&mut self, fertilizer_id: &String) {
        match self.browser.find(fertilizer_id) {
            Some(fertilizer) => {
                self.selected_set.add_selected_fertilizer(fertilizer);
            }
            None => {
                // some notification maybe
            }
        }
    }

    pub fn update_fertilizer_amount(&mut self, fertilizer_id: &String, amount: f64) {
        self.selected_set
            .update_selected_fertilizer(fertilizer_id, amount);
    }

    pub fn exclude_fertilizer(&mut self, fertilizer_id: &String) {
        self.selected_set.exclude_fertilizer(fertilizer_id);
    }
}
