#[derive(Debug, Clone)]
pub struct Element {
    pub symbol: &'static str,
    pub atomic_weight: f32,
    pub nutrient: bool,
}

impl Element {
    pub fn new(symbol: &'static str, atomic_weight: f32) -> Self {
        Self {
            symbol,
            atomic_weight,
            nutrient: false,
        }
    }

    pub fn nutrient(mut self) -> Self {
        self.nutrient = true;

        self
    }

    pub fn is_nutrient(&self) -> bool {
        self.nutrient
    }
}
