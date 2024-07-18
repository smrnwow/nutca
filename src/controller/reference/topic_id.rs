#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TopicId {
    Profiles,
    ProfileEditor,
    Fertilizers,
    FertilizerEditor,
    Solutions,
    NutrientSolutionEditor,
    StockSolutionEditor,
}

impl ToString for TopicId {
    fn to_string(&self) -> String {
        match self {
            Self::Profiles => String::from("profiles"),
            Self::ProfileEditor => String::from("profile-editor"),
            Self::Fertilizers => String::from("fertilizers"),
            Self::FertilizerEditor => String::from("fertilizer-editor"),
            Self::Solutions => String::from("solutions"),
            Self::NutrientSolutionEditor => String::from("nutrient-solution-editor"),
            Self::StockSolutionEditor => String::from("stock-solution-editor"),
        }
    }
}
