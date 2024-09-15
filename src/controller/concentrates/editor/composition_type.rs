#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CompositionType {
    FromFertilizers,
    FromSolution,
}

impl CompositionType {
    pub fn label(&self) -> String {
        match self {
            Self::FromFertilizers => String::from("из удобрений"),
            Self::FromSolution => String::from("из раствора"),
        }
    }
}

impl ToString for CompositionType {
    fn to_string(&self) -> String {
        match self {
            Self::FromFertilizers => String::from("from-fertilizers"),
            Self::FromSolution => String::from("from-solution"),
        }
    }
}

impl From<String> for CompositionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "from-solution" => Self::FromSolution,
            _ => Self::FromFertilizers,
        }
    }
}
