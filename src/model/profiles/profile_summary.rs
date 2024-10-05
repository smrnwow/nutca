use super::Profile;

pub struct ProfileSummary {
    pub id: String,
    pub name: String,
}

impl From<Profile> for ProfileSummary {
    fn from(profile: Profile) -> Self {
        Self {
            id: String::new(),
            name: String::new(),
        }
    }
}
