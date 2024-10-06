use crate::model::chemistry::NutrientAmount;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NutrientContentValueProps {
    nutrient: NutrientAmount,
}

#[component]
pub fn NutrientContentValue(props: NutrientContentValueProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.nutrient.nutrient().symbol(),
            value: Signal::new(props.nutrient.value()),
        }
    }
}
