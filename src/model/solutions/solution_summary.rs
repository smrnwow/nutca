#[derive(Clone, Debug, PartialEq)]
pub struct SolutionSummary {
    id: String,
    name: String,
}

impl SolutionSummary {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
