#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SourceType {
    Label,
    Formula,
}

impl SourceType {
    pub fn label(&self) -> String {
        match self {
            Self::Label => String::from("С этикетки"),
            Self::Formula => String::from("По формуле"),
        }
    }

    pub fn value(&self) -> String {
        match self {
            Self::Label => String::from("label"),
            Self::Formula => String::from("formula"),
        }
    }
}

impl From<String> for SourceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "label" => Self::Label,
            "formula" => Self::Formula,
            _ => Self::Label,
        }
    }
}
