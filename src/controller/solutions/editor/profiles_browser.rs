use crate::model::profiles::Profile;
use crate::repository::ProfilesRepository;

#[derive(Clone, Debug, PartialEq)]
pub struct ProfilesBrowser {
    profiles_repository: ProfilesRepository,
    search_query: String,
    page_index: usize,
    limit: usize,
}

impl ProfilesBrowser {
    pub fn new(profiles_repository: ProfilesRepository) -> Self {
        Self {
            profiles_repository,
            search_query: String::new(),
            page_index: 1,
            limit: 10,
        }
    }

    pub fn fetch(&self) -> Vec<Profile> {
        self.profiles_repository
            .search(&self.search_query, self.limit, self.page_index)
    }

    pub fn find(&self, profile_id: &String) -> Option<Profile> {
        self.profiles_repository.find(profile_id)
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }
}
