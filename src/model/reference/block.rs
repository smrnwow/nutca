use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Block {
    title: String,
    text: String,
}

impl Block {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            text: String::new(),
        }
    }

    pub fn with_title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}
