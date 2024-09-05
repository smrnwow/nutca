use crate::model::concentrates::parts::ManualPart;
use crate::model::fertilizers::Fertilizer;
use crate::repository::Storage;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersBrowser {
    storage: Signal<Storage>,
    excluded_ids: Vec<String>,
    search_query: String,
    page_index: usize,
    limit: usize,
    fertilizers: HashMap<String, Fertilizer>,
}

impl FertilizersBrowser {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            excluded_ids: Vec::new(),
            search_query: String::new(),
            page_index: 1,
            limit: 10,
            fertilizers: HashMap::new(),
        }
    }

    pub fn load_fertilizers(&mut self, fertilizers_ids: Vec<&String>) {
        /*
        let fertilizers = match self.storage.read().fertilizers().filter(&fertilizers_ids) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        };

        fertilizers.into_iter().for_each(|fertilizer| {
            self.used.insert(fertilizer.id(), fertilizer);
        });
        */
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
            self.excluded_ids.as_ref(),
            self.limit,
            self.limit * (self.page_index - 1),
        ) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }

    pub fn refresh(&mut self) {}

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn exclude_many(&mut self, fertilizers_ids: Vec<String>) {
        for fertilizer_id in fertilizers_ids {
            self.excluded_ids.push(fertilizer_id);
        }
    }

    pub fn exclude(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        match self.find(&fertilizer_id) {
            Some(fertilizer) => {
                self.excluded_ids.push(fertilizer_id);
                Some(fertilizer)
            }
            None => None,
        }
    }

    pub fn include(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        match self.excluded_ids.iter().position(|id| *id == fertilizer_id) {
            Some(index) => {
                self.excluded_ids.remove(index);
                self.find(&fertilizer_id)
            }
            None => None,
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn update_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    fn find(&self, fertilizer_id: &str) -> Option<Fertilizer> {
        match self.storage.read().fertilizers().get(fertilizer_id) {
            Ok(fertilizer) => Some(fertilizer),
            Err(_) => None,
        }
    }
}
