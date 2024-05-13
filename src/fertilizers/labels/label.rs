use super::component::Component;
use super::nutrient::Nutrient;
use super::units::Units;
use crate::chemistry::table::Table;

#[derive(Debug)]
pub struct Label {
    table: Table,
    composition: Vec<Component>,
    nutrients: Vec<Nutrient>,
    pub units: Units,
}

impl Label {
    pub fn new(units: Units) -> Self {
        let label = Self {
            table: Table::new(),
            units,
            nutrients: Vec::new(),
            composition: Vec::new(),
        };

        label
    }

    pub fn add_nutrient(&mut self, nutrient: Nutrient) {
        match self.units {
            Units::WeightVolume => {
                self.composition.push(Component {
                    element: self.table.element(nutrient.symbol().as_str()).unwrap(),
                    percent: nutrient.amount() / 10000.,
                });

                self.nutrients.push(nutrient);
            }
            Units::Percentage => {
                self.composition.push(Component {
                    element: self.table.element(nutrient.symbol().as_str()).unwrap(),
                    percent: nutrient.amount(),
                });

                self.nutrients.push(nutrient);
            }
        }
    }

    pub fn components(&self) -> Vec<&Component> {
        self.composition.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Label;
    use super::Nutrient;
    use super::Units;

    #[test]
    fn percentage_label() {
        let mut label = Label::new(Units::Percentage);

        label.add_nutrient(Nutrient::Nitrogen(Some(19.0), None, None));
        label.add_nutrient(Nutrient::Phosphorus(None, Some(6.0)));
        label.add_nutrient(Nutrient::Pottasium(None, Some(20.0)));
        label.add_nutrient(Nutrient::Magnesium(None, Some(3.0)));
        label.add_nutrient(Nutrient::Sulfur(Some(3.0), None, None));
        label.add_nutrient(Nutrient::Iron(Some(0.07)));
        label.add_nutrient(Nutrient::Manganese(Some(0.04)));
        label.add_nutrient(Nutrient::Boron(Some(0.025)));
        label.add_nutrient(Nutrient::Cuprum(Some(0.01)));
        label.add_nutrient(Nutrient::Zink(Some(0.025)));
        label.add_nutrient(Nutrient::Molibden(Some(0.004)));

        // println!("label {:#?}", label);
    }

    #[test]
    fn weight_volume_label() {
        let mut label = Label::new(Units::WeightVolume);

        label.add_nutrient(Nutrient::Magnesium(Some(15000.), None));
        label.add_nutrient(Nutrient::Iron(Some(3200.)));
        label.add_nutrient(Nutrient::Manganese(Some(1600.)));
        label.add_nutrient(Nutrient::Boron(Some(1200.)));
        label.add_nutrient(Nutrient::Zink(Some(360.)));
        label.add_nutrient(Nutrient::Cuprum(Some(320.)));
        label.add_nutrient(Nutrient::Molibden(Some(102.)));

        // println!("label {:#?}", label);
    }
}
