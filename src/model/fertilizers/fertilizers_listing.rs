use crate::model::fertilizers::Fertilizer;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersListing {
    fertilizers: Vec<Fertilizer>,
    search_query: String,
}

impl FertilizersListing {
    pub fn new(fertilizers: Vec<Fertilizer>) -> Self {
        Self {
            fertilizers,
            search_query: String::new(),
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn list(&self) -> Vec<Fertilizer> {
        if self.search_query.len() == 0 {
            self.fertilizers.clone()
        } else {
            self.fertilizers
                .iter()
                .filter(|fertilizer| {
                    fertilizer
                        .name()
                        .to_lowercase()
                        .contains(self.search_query.as_str())
                })
                .map(|fertilizer| fertilizer.clone())
                .collect()
        }
    }
}
