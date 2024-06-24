use crate::model::fertilizers::Fertilizer;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersListing {
    fertilizers: Vec<Fertilizer>,
    excluded_fertilizers_ids: Vec<String>,
    search_query: String,
}

impl FertilizersListing {
    pub fn new(fertilizers: Vec<Fertilizer>) -> Self {
        Self {
            fertilizers,
            excluded_fertilizers_ids: Vec::new(),
            search_query: String::new(),
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn list(&self) -> Vec<Fertilizer> {
        self.fertilizers
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
            .collect()
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

    pub fn include(&mut self, fertilizer_id: String) {
        self.excluded_fertilizers_ids = self
            .excluded_fertilizers_ids
            .iter()
            .cloned()
            .filter(|excluded_fertilizer_id| *excluded_fertilizer_id != fertilizer_id)
            .collect();
    }
}
