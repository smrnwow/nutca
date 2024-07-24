use crate::ui::components::NutrientValue;
use dioxus::prelude::*;
use nutca::chemistry::NutrientAmount;

#[derive(Props, PartialEq, Clone)]
pub struct NutrientContentValueProps {
    nutrient: NutrientAmount,
}

#[component]
pub fn NutrientContentValue(props: NutrientContentValueProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.nutrient.nutrient().symbol(),
            value: props.nutrient.value(),
        }
    }
}
