use crate::repository::Storage;
use dioxus::prelude::*;
use nutca::fertilizers::Fertilizer;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersListing {
    storage: Signal<Storage>,
    fertilizers: HashMap<String, Fertilizer>,
    items: Vec<String>,
    excluded_ids: Vec<String>,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl FertilizersListing {
    pub fn new(storage: Signal<Storage>) -> Self {
        let fertilizers = match storage.read().fertilizers().list() {
            Ok(list) => list.iter().fold(HashMap::new(), |mut acc, fertilizer| {
                acc.insert(fertilizer.id(), fertilizer.clone());
                acc
            }),
            Err(_) => HashMap::new(),
        };

        let items: Vec<String> = fertilizers
            .keys()
            .map(|fertilizer_id| fertilizer_id.clone())
            .collect();

        Self {
            storage,
            fertilizers,
            items,
            excluded_ids: Vec::new(),
            search_query: String::new(),
            page_index: 1,
            limit: 8,
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();

        self.update_list();
    }

    pub fn list(&self) -> Vec<Fertilizer> {
        let start = (self.page_index - 1) * self.limit;

        let end = (self.page_index * self.limit) - 1;

        if end < self.items.len() {
            return self.items[start..=end]
                .iter()
                .map(|fertilizer_id| self.fertilizers.get(fertilizer_id).unwrap().clone())
                .collect();
        } else {
            return self.items[start..]
                .iter()
                .map(|fertilizer_id| self.fertilizers.get(fertilizer_id).unwrap().clone())
                .collect();
        }
    }

    pub fn find(&self, fertilizer_id: String) -> Option<Fertilizer> {
        match self.fertilizers.get(&fertilizer_id) {
            Some(fertilizer) => Some(fertilizer.clone()),
            None => None,
        }
    }

    pub fn excluded(&mut self, fertilizers_ids: Vec<String>) {
        for fertilizer_id in fertilizers_ids {
            self.excluded_ids.push(fertilizer_id);
        }

        self.update_list();
    }

    pub fn exclude(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        self.excluded_ids.push(fertilizer_id.clone());

        self.update_list();

        self.find(fertilizer_id)
    }

    pub fn include(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        self.excluded_ids = self
            .excluded_ids
            .iter()
            .cloned()
            .filter(|excluded_fertilizer_id| *excluded_fertilizer_id != fertilizer_id)
            .collect();

        self.update_list();

        self.find(fertilizer_id)
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn update_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn total(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }

    fn update_list(&mut self) {
        self.items = self
            .fertilizers
            .values()
            .filter(|fertilizer| {
                let search_filter = fertilizer
                    .name()
                    .to_lowercase()
                    .contains(self.search_query.as_str());

                let exclusion_filter = !self.excluded_ids.contains(&fertilizer.id());

                search_filter && exclusion_filter
            })
            .map(|fertilizer| fertilizer.id())
            .collect();

        let pages_count = self.items.len() / self.limit;

        if pages_count < self.page_index {
            self.page_index = 1;
        }
    }
}
