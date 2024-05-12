use super::measurement_unit::MeasurementUnit;

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub units: MeasurementUnit,
    pub nutrients: HashMap<String, NutrientLabel>,
}

impl Label {
    pub fn new(name: &str, units: MeasurementUnit, nutrients: Vec<(&str, f32)>) -> Label {
        let mut label = Label {
            name: String::from(name),
            units,
            nutrients: HashMap::new(),
        };

        for (symbol, amount) in nutrients {
            label.add_nutrient(symbol, amount);
        }

        label
    }

    fn add_nutrient(&mut self, symbol: &str, amount: f32) {
        let mut parser = parsing::Parser::new();

        let compound = parser.parse(symbol);

        for component in compound.list_components() {
            if component.element.nutrient {
                match self.nutrients.get_mut(component.element.symbol.as_str()) {
                    Some(nutrient) => {
                        nutrient.add((component.mass_percent / 100.) * amount);
                    }

                    None => {
                        self.nutrients.insert(
                            component.element.symbol.to_string(),
                            NutrientLabel::new(
                                component.element.symbol.as_str(),
                                (component.mass_percent / 100.) * amount,
                            ),
                        );
                    }
                }
            }
        }
    }
}
