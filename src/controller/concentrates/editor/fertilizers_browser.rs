use crate::model::concentrates::parts::ManualPart;
use crate::model::fertilizers::Fertilizer;
use crate::repository::Storage;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersBrowser {
    storage: Signal<Storage>,
    search_query: String,
    page_index: usize,
    limit: usize,
    fertilizers: HashMap<String, Fertilizer>,
}

impl FertilizersBrowser {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
            fertilizers: HashMap::new(),
        }
    }

    pub fn with_fertilizers(&mut self, fertilizers_ids: Vec<String>) {
        let fertilizers = match self.storage.read().fertilizers().filter(&fertilizers_ids) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        };

        fertilizers.into_iter().for_each(|fertilizer| {
            self.fertilizers.insert(fertilizer.id(), fertilizer);
        });
    }

    pub fn list(&self, part: &ManualPart) -> Vec<Signal<(Fertilizer, f64)>> {
        let mut list = Vec::new();

        part.fertilizers().iter().for_each(|fertilizer_amount| {
            if let Some(fertilizer) = self.fertilizers.get(&fertilizer_amount.id()) {
                list.push(Signal::new((
                    fertilizer.clone(),
                    fertilizer_amount.amount(),
                )));
            }
        });

        list
    }

    pub fn get(&mut self, fertilizer_id: &String) -> Option<Fertilizer> {
        match self.fertilizers.get(fertilizer_id) {
            Some(fertilizer) => Some(fertilizer.clone()),
            None => match self.storage.read().fertilizers().get(fertilizer_id) {
                Ok(fertilizer) => {
                    self.fertilizers
                        .insert(fertilizer_id.clone(), fertilizer.clone());

                    Some(fertilizer)
                }
                Err(_) => None,
            },
        }
    }

    pub fn fetch(&self) -> Vec<Fertilizer> {
        match self.storage.read().fertilizers().search(
            &self.search_query,
            &[],
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }
}
