use crate::model::profiles::Profile;
use crate::repository::NutritionProgramsRepository;

#[derive(Clone, Debug, PartialEq)]
pub struct NutritionProgramBrowser {
    nutrition_programs_repository: NutritionProgramsRepository,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl NutritionProgramBrowser {
    pub fn new(nutrition_programs_repository: NutritionProgramsRepository) -> Self {
        Self {
            nutrition_programs_repository,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
        }
    }

    pub fn fetch(&self) -> Vec<Profile> {
        self.nutrition_programs_repository
            .search(&self.search_query, self.limit, self.page_index)
    }

    pub fn find(&self, profile_id: &String) -> Option<Profile> {
        self.nutrition_programs_repository.find(profile_id)
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }
}
