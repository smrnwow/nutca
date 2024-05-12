pub struct Composition<'a> {
    nutrients: Vec<(&'a str, f32)>,
}

impl<'a> Composition<'a> {
    pub fn new() -> Self {
        Self {
            nutrients: Vec::new(),
        }
    }

    pub fn add_nutrient(&mut self, nutrient: &'a str, amount: f32) {
        self.nutrients.push((nutrient, amount));
    }

    pub fn nutrients(&self) -> &Vec<(&str, f32)> {
        &self.nutrients
    }
}
