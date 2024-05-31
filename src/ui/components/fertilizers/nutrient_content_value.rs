use crate::model::chemistry::NutrientAmount;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NutrientContentValueProps {
    nutrient_amount: NutrientAmount,
}

#[component]
pub fn NutrientContentValue(props: NutrientContentValueProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.nutrient_amount.symbol(),
            value: props.nutrient_amount.value(),
        }
    }
}
