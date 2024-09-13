use crate::model::fertilizers::Fertilizer;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersRepository {
    storage: Signal<Storage>,
}

impl FertilizersRepository {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self { storage }
    }

    pub fn find_by_ids(&self, fertilizers_ids: Vec<&String>) -> Vec<Fertilizer> {
        let fertilizers_ids: Vec<String> = fertilizers_ids.into_iter().cloned().collect();

        match self.storage.read().fertilizers().filter(&fertilizers_ids) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }

    pub fn search(
        &self,
        query: &String,
        projection: &Vec<String>,
        limit: usize,
        page_index: usize,
    ) -> Vec<Fertilizer> {
        let proj: Vec<&String> = projection.iter().collect();

        match self.storage.read().fertilizers().search(
            query,
            &proj,
            limit,
            limit * (page_index - 1),
        ) {
            Ok(fertilizers) => fertilizers,
            Err(_) => Vec::new(),
        }
    }

    pub fn find(&self, fertilizer_id: &String) -> Option<Fertilizer> {
        match self.storage.read().fertilizers().get(fertilizer_id) {
            Ok(fertilizer) => Some(fertilizer),
            Err(_) => None,
        }
    }
}
