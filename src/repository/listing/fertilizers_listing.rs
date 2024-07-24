use crate::repository::Storage;
use dioxus::prelude::*;
use nutca::fertilizers::Fertilizer;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersListing {
    storage: Signal<Storage>,
    fertilizers: Vec<Fertilizer>,
    excluded_fertilizers_ids: Vec<String>,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl FertilizersListing {
    pub fn new(storage: Signal<Storage>) -> Self {
        let fertilizers = match storage.read().fertilizers().list() {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        };

        Self {
            storage,
            fertilizers,
            excluded_fertilizers_ids: Vec::new(),
            search_query: String::new(),
            page_index: 1,
            limit: 8,
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn list(&self) -> Vec<Fertilizer> {
        let fertilizers: Vec<Fertilizer> = self
            .fertilizers
            .iter()
            .cloned()
            .filter(|fertilizer| {
                let search_filter = fertilizer
                    .name()
                    .to_lowercase()
                    .contains(self.search_query.as_str());

                let exclusion_filter = !self.excluded_fertilizers_ids.contains(&fertilizer.id());

                search_filter && exclusion_filter
            })
            .collect();

        let start = (self.page_index - 1) * self.limit;

        let end = (self.page_index * self.limit) - 1;

        if end < fertilizers.len() {
            Vec::from(&fertilizers[start..=end])
        } else {
            Vec::from(&fertilizers[start..])
        }
    }

    pub fn find(&self, fertilizer_id: String) -> Option<Fertilizer> {
        let fertilizer = self
            .fertilizers
            .iter()
            .find(|fertilizer| fertilizer.id() == fertilizer_id);

        match fertilizer {
            Some(fertilizer) => Some(fertilizer.clone()),
            None => None,
        }
    }

    pub fn exclude(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        self.excluded_fertilizers_ids.push(fertilizer_id.clone());

        self.find(fertilizer_id)
    }

    pub fn include(&mut self, fertilizer_id: String) -> Option<Fertilizer> {
        self.excluded_fertilizers_ids = self
            .excluded_fertilizers_ids
            .iter()
            .cloned()
            .filter(|excluded_fertilizer_id| *excluded_fertilizer_id != fertilizer_id)
            .collect();

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
        self.fertilizers
            .iter()
            .filter(|fertilizer| {
                let search_filter = fertilizer
                    .name()
                    .to_lowercase()
                    .contains(self.search_query.as_str());

                let exclusion_filter = !self.excluded_fertilizers_ids.contains(&fertilizer.id());

                search_filter && exclusion_filter
            })
            .collect::<Vec<&Fertilizer>>()
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }
}
