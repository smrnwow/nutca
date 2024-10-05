use crate::model::chemistry::Nutrients;
use crate::model::solutions::ProfileRequirement;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ProfileRequirementSchema {
    Saved(String, String),
    ByValue(Nutrients),
}

impl From<ProfileRequirement> for ProfileRequirementSchema {
    fn from(profile_requirement: ProfileRequirement) -> Self {
        match profile_requirement.profile() {
            Some((profile, stage_id)) => Self::Saved(profile.id().to_string(), stage_id.clone()),
            None => Self::ByValue(*profile_requirement.nutrients()),
        }
    }
}

impl Default for ProfileRequirementSchema {
    fn default() -> Self {
        Self::ByValue(Nutrients::new())
    }
}
