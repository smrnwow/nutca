use super::NutrientPercent;

#[derive(Debug)]
pub struct Fertiliser {
    name: String,
    vendor: String,
    nutrients: Vec<NutrientPercent>,
}

impl Fertiliser {
    pub fn new() -> Self {
        Self {
            name: String::from(""),
            vendor: String::from(""),
            nutrients: Vec::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn vendor(mut self, vendor: &str) -> Self {
        self.vendor = vendor.to_string();
        self
    }

    pub fn add_nutrient(&mut self, nutrient: NutrientPercent) {
        self.nutrients.push(nutrient);
    }

    pub fn nutrients(&self) -> &Vec<NutrientPercent> {
        &self.nutrients
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
