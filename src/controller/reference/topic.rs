use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Topic {
    id: String,
    title: String,
    text: String,
}

impl Topic {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            text: String::new(),
        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn with_title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}
