use super::{NutrientPercent, Units};

#[derive(Debug)]
pub enum Component {
    Nitrogen(Option<f64>),
    NitrogenNitrate(Option<f64>),
    NitrogenAmmonium(Option<f64>),
    Phosphorus(Option<f64>, Option<f64>),
    Potassium(Option<f64>, Option<f64>),
    Calcium(Option<f64>, Option<f64>),
    Magnesium(Option<f64>, Option<f64>),
    Sulfur(Option<f64>, Option<f64>, Option<f64>),
    Iron(Option<f64>),
    Manganese(Option<f64>),
    Boron(Option<f64>),
    Copper(Option<f64>),
    Zink(Option<f64>),
    Molybdenum(Option<f64>),
}

impl Component {
    pub fn nutrient_percent(&self, units: Units) -> NutrientPercent {
        let percent = match units {
            Units::WeightVolume => self.amount() / 10000.,
            Units::Percentage => self.amount(),
        };

        match self {
            Component::Nitrogen(_) => NutrientPercent::Nitrogen(percent),
            Component::NitrogenNitrate(_) => NutrientPercent::NitrogenNitrate(percent),
            Component::NitrogenAmmonium(_) => NutrientPercent::NitrogenAmmonium(percent),
            Component::Phosphorus(_, _) => NutrientPercent::Phosphorus(percent),
            Component::Potassium(_, _) => NutrientPercent::Potassium(percent),
            Component::Calcium(_, _) => NutrientPercent::Calcium(percent),
            Component::Magnesium(_, _) => NutrientPercent::Magnesium(percent),
            Component::Sulfur(_, _, _) => NutrientPercent::Sulfur(percent),
            Component::Iron(_) => NutrientPercent::Iron(percent),
            Component::Boron(_) => NutrientPercent::Boron(percent),
            Component::Manganese(_) => NutrientPercent::Manganese(percent),
            Component::Zink(_) => NutrientPercent::Zink(percent),
            Component::Copper(_) => NutrientPercent::Copper(percent),
            Component::Molybdenum(_) => NutrientPercent::Molybdenum(percent),
        }
    }

    fn amount(&self) -> f64 {
        match self {
            Component::Nitrogen(total) => {
                return total.unwrap_or(0.0);
            }
            Component::NitrogenNitrate(amount) => {
                return amount.unwrap_or(0.0);
            }
            Component::NitrogenAmmonium(amount) => {
                return amount.unwrap_or(0.0);
            }
            Component::Phosphorus(elemental, pentoxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(pentoxide) = pentoxide {
                    amount += pentoxide * 0.436421;
                }

                amount
            }
            Component::Potassium(elemental, oxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(oxide) = oxide {
                    amount += oxide * 0.830148;
                }

                amount
            }
            Component::Calcium(elemental, oxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(oxide) = oxide {
                    amount += oxide * 0.714691;
                }

                amount
            }
            Component::Magnesium(elemental, oxide) => {
                let mut amount = 0.0;

                if let Some(elemental) = elemental {
                    amount += elemental;
                }

                if let Some(oxide) = oxide {
                    amount += oxide * 0.603036;
                }

                amount
            }
            Component::Sulfur(elemental, trioxide, tetroxide) => {
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
            Component::Iron(elemental) => {
                return elemental.unwrap_or(0.0);
            }
            Component::Boron(elemental) => {
                return elemental.unwrap_or(0.0);
            }
            Component::Manganese(elemental) => {
                return elemental.unwrap_or(0.0);
            }
            Component::Zink(elemental) => {
                return elemental.unwrap_or(0.0);
            }
            Component::Copper(elemental) => {
                return elemental.unwrap_or(0.0);
            }
            Component::Molybdenum(elemental) => {
                return elemental.unwrap_or(0.0);
            }
        }
    }
}
