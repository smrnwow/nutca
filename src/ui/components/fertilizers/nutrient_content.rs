use crate::model::fertilizers::NutrientContent;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NutrientContentProps {
    nutrient_content: NutrientContent,
}

#[component]
pub fn NutrientContent(props: NutrientContentProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.nutrient_content.symbol(),
            value: props.nutrient_content.value(),
        }
    }
}
