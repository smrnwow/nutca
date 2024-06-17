use crate::model::chemistry::Nutrient;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NutrientContentValueProps {
    nutrient: Nutrient,
}

#[component]
pub fn NutrientContentValue(props: NutrientContentValueProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.nutrient.symbol(),
            value: props.nutrient.value(),
        }
    }
}
