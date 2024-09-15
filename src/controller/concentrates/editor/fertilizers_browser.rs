use crate::model::fertilizers::Fertilizer;
use crate::repository::FertilizersRepository;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersBrowser {
    fertilizers_repository: FertilizersRepository,
    search_query: String,
    page_index: usize,
    limit: usize,
    fertilizers: HashMap<String, Fertilizer>,
}

impl FertilizersBrowser {
    pub fn new(fertilizers_repository: FertilizersRepository) -> Self {
        Self {
            fertilizers_repository,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
            fertilizers: HashMap::new(),
        }
    }

    pub fn get(&mut self, fertilizer_id: &String) -> Option<Fertilizer> {
        self.fertilizers_repository.find(fertilizer_id)
    }

    pub fn fetch(&self) -> Vec<Fertilizer> {
        self.fertilizers_repository.search(
            &self.search_query,
            &Vec::new(),
            self.limit,
            self.page_index,
        )
    }
}
