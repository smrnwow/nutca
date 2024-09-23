use crate::model::chemistry::Nutrients;
use uuid::Uuid;

pub struct Stage {
    id: String,
    name: String,
    nutrients: Nutrients,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            nutrients: Nutrients::new(),
        }
    }
}
