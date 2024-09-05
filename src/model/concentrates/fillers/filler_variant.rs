#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FillerVariant {
    Auto,
    Manual,
}

impl ToString for FillerVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => String::from("auto"),
            Self::Manual => String::from("manual"),
        }
    }
}

impl From<String> for FillerVariant {
    fn from(value: String) -> Self {
        match value.as_str() {
            "manual" => Self::Manual,
            _ => Self::Auto,
        }
    }
}
