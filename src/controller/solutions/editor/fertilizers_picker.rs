use crate::model::fertilizers::Fertilizer;
use crate::repository::FertilizersRepository;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersPicker {
    fertilizers_repository: FertilizersRepository,
    search_query: String,
    limit: usize,
    page_index: usize,
    projection: Vec<String>,
}

impl FertilizersPicker {
    pub fn new(fertilizers_repository: FertilizersRepository) -> Self {
        Self {
            fertilizers_repository,
            search_query: String::new(),
            limit: 8,
            page_index: 1,
            projection: Vec::new(),
        }
    }

    pub fn browse(&self) -> Vec<Fertilizer> {
        self.fertilizers_repository.search(
            &self.search_query,
            &self.projection,
            self.limit,
            self.page_index,
        )
    }

    pub fn pick(&mut self, fertilizer_id: &String) -> Option<Fertilizer> {
        match self.fertilizers_repository.find(fertilizer_id) {
            Some(fertilizer) => {
                self.projection.push(fertilizer.id());
                Some(fertilizer)
            }

            None => None,
        }
    }

    pub fn set_projection(&mut self, fertilizers_ids: Vec<&String>) -> &mut Self {
        self.projection = fertilizers_ids.into_iter().cloned().collect();
        self
    }

    pub fn remove_projection(&mut self, fertilizer_id: &String) {
        let position = self
            .projection
            .iter()
            .position(|projection| **projection == *fertilizer_id);

        if let Some(index) = position {
            self.projection.remove(index);
        }
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }
}
