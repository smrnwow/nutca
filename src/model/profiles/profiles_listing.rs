use crate::model::profiles::Profile;

#[derive(Clone, Debug, PartialEq)]
pub struct ProfilesListing {
    profiles: Vec<Profile>,
    search_query: String,
    page_index: usize,
    limit: Option<usize>,
}

impl ProfilesListing {
    pub fn new(profiles: Vec<Profile>) -> Self {
        Self {
            profiles,
            search_query: String::new(),
            page_index: 1,
            limit: None,
        }
    }

    pub fn search(&mut self, search_query: String) {
        self.search_query = search_query.to_lowercase();
    }

    pub fn find(&self, profile_id: String) -> Option<Profile> {
        let profile = self
            .profiles
            .iter()
            .find(|profile| profile.id() == profile_id);

        match profile {
            Some(profile) => Some(profile.clone()),
            None => None,
        }
    }

    pub fn list(&self) -> Vec<Profile> {
        let profiles: Vec<Profile> = self
            .profiles
            .iter()
            .filter(|profile| {
                profile
                    .name()
                    .to_lowercase()
                    .contains(self.search_query.as_str())
            })
            .map(|profile| profile.clone())
            .collect();

        if let Some(limit) = self.limit {
            let start = (self.page_index - 1) * limit;

            let end = (self.page_index * limit) - 1;

            if end < profiles.len() {
                Vec::from(&profiles[start..=end])
            } else {
                Vec::from(&profiles[start..])
            }
        } else {
            profiles
        }
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn update_limit(&mut self, limit: Option<usize>) {
        self.limit = limit;
    }

    pub fn total(&self) -> usize {
        self.profiles
            .iter()
            .filter(|profile| {
                profile
                    .name()
                    .to_lowercase()
                    .contains(self.search_query.as_str())
            })
            .collect::<Vec<&Profile>>()
            .len()
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        match self.limit {
            Some(limit) => limit,
            None => self.total(),
        }
    }

    pub fn search_query(&self) -> String {
        self.search_query.clone()
    }
}
