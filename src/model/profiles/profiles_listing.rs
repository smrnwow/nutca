use crate::model::profiles::Profile;

#[derive(Clone, Debug, PartialEq)]
pub struct ProfilesListing {
    profiles: Vec<Profile>,
    search_query: String,
}

impl ProfilesListing {
    pub fn new(profiles: Vec<Profile>) -> Self {
        Self {
            profiles,
            search_query: String::new(),
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
        if self.search_query.len() == 0 {
            self.profiles.clone()
        } else {
            self.profiles
                .iter()
                .filter(|profile| {
                    profile
                        .name()
                        .to_lowercase()
                        .contains(self.search_query.as_str())
                })
                .map(|profile| profile.clone())
                .collect()
        }
    }
}
