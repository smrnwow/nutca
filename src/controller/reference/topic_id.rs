#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TopicId {
    ProfilesDashboard,
    ProfileEditor,
    FertilizersDashboard,
    FertilizerEditor,
    SolutionsDashboard,
    NutrientSolutionEditor,
    StockSolutionEditor,
}

impl TopicId {
    pub fn title(&self) -> String {
        match self {
            Self::ProfilesDashboard => String::from("Профили питания"),
            Self::ProfileEditor => String::from("Редактор профиля питания"),
            Self::FertilizersDashboard => String::from("Удобрения"),
            Self::FertilizerEditor => String::from("Редактор удобрения"),
            Self::SolutionsDashboard => String::from("Растворы"),
            Self::NutrientSolutionEditor => String::from("Редактор питательного раствора"),
            Self::StockSolutionEditor => String::from("Редактор рабочего раствора"),
        }
    }
}

impl ToString for TopicId {
    fn to_string(&self) -> String {
        match self {
            Self::ProfilesDashboard => String::from("profiles-dashboard"),
            Self::ProfileEditor => String::from("profile-editor"),
            Self::FertilizersDashboard => String::from("fertilizers-dashboard"),
            Self::FertilizerEditor => String::from("fertilizer-editor"),
            Self::SolutionsDashboard => String::from("solutions-dashboard"),
            Self::NutrientSolutionEditor => String::from("nutrient-solution-editor"),
            Self::StockSolutionEditor => String::from("stock-solution-editor"),
        }
    }
}
