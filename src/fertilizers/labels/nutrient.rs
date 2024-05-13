#[derive(Debug)]
pub enum Nutrient {
    Nitrogen(Option<f32>, Option<f32>, Option<f32>),
    Phosphorus(Option<f32>, Option<f32>),
    Pottasium(Option<f32>, Option<f32>),
    Calcium(Option<f32>, Option<f32>),
    Magnesium(Option<f32>, Option<f32>),
    Sulfur(Option<f32>, Option<f32>, Option<f32>),
    Iron(Option<f32>),
    Manganese(Option<f32>),
    Boron(Option<f32>),
    Cuprum(Option<f32>),
    Zink(Option<f32>),
    Molibden(Option<f32>),
}

impl Nutrient {
    pub fn amount(&self) -> f32 {
        match self {
            Nutrient::Nitrogen(total, _, _) => total.unwrap_or(0.0),
            Nutrient::Phosphorus(elemental, pentoxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(pentoxide) = pentoxide {
                    amount += pentoxide * 0.436421;
                }

                amount
            }
            Nutrient::Pottasium(elemental, oxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(oxide) = oxide {
                    amount += oxide * 0.830148;
                }

                amount
            }
            Nutrient::Calcium(elemental, oxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(oxide) = oxide {
                    amount += oxide * 0.714691;
                }

                amount
            }
            Nutrient::Magnesium(elemental, oxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(oxide) = oxide {
                    amount += oxide * 0.603036;
                }

                amount
            }
            Nutrient::Sulfur(elemental, trioxide, tetroxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(trioxide) = trioxide {
                    amount += trioxide * 0.400496;
                }

                if let Some(tetroxide) = tetroxide {
                    amount += tetroxide * 0.333793;
                }

                amount
            }
            Nutrient::Iron(elemental) => elemental.unwrap_or(0.0),
            Nutrient::Boron(elemental) => elemental.unwrap_or(0.0),
            Nutrient::Manganese(elemental) => elemental.unwrap_or(0.0),
            Nutrient::Zink(elemental) => elemental.unwrap_or(0.0),
            Nutrient::Cuprum(elemental) => elemental.unwrap_or(0.0),
            Nutrient::Molibden(elemental) => elemental.unwrap_or(0.0),
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Nutrient::Nitrogen(_, _, _) => String::from("N"),
            Nutrient::Phosphorus(_, _) => String::from("P"),
            Nutrient::Pottasium(_, _) => String::from("K"),
            Nutrient::Calcium(_, _) => String::from("Ca"),
            Nutrient::Magnesium(_, _) => String::from("Mg"),
            Nutrient::Sulfur(_, _, _) => String::from("S"),
            Nutrient::Iron(_) => String::from("Fe"),
            Nutrient::Boron(_) => String::from("B"),
            Nutrient::Manganese(_) => String::from("Mn"),
            Nutrient::Zink(_) => String::from("Zn"),
            Nutrient::Cuprum(_) => String::from("Cu"),
            Nutrient::Molibden(_) => String::from("Mo"),
        }
    }
}
