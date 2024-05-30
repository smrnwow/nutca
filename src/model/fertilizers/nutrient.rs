#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Nutrient {
    Nitrogen,
    NitrogenNitrate,
    NitrogenAmmonium,
    Phosphor,
    Potassium,
    Calcium,
    Magnesium,
    Sulfur,
    Iron,
    Manganese,
    Copper,
    Zinc,
    Boron,
    Molybdenum,
}

impl Nutrient {
    pub fn symbol(&self) -> &str {
        match self {
            Nutrient::Nitrogen => "N",
            Nutrient::NitrogenNitrate => "NO3",
            Nutrient::NitrogenAmmonium => "NH4",
            Nutrient::Phosphor => "P",
            Nutrient::Potassium => "K",
            Nutrient::Calcium => "Ca",
            Nutrient::Magnesium => "Mg",
            Nutrient::Sulfur => "S",
            Nutrient::Iron => "Fe",
            Nutrient::Manganese => "Mn",
            Nutrient::Copper => "Cu",
            Nutrient::Zinc => "Zn",
            Nutrient::Boron => "B",
            Nutrient::Molybdenum => "Mo",
        }
    }
}
