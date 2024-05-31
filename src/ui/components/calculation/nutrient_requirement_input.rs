use crate::model::chemistry::NutrientAmount;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NutrientRequirementInputProps {
    nutrient_amount: NutrientAmount,
    on_update: Option<EventHandler<NutrientAmount>>,
}

#[component]
pub fn NutrientRequirementInput(props: NutrientRequirementInputProps) -> Element {
    rsx! {
        if let Some(on_update) = props.on_update {
            NutrientValue {
                symbol: props.nutrient_amount.symbol(),
                value: props.nutrient_amount.value(),
                on_change: move |value| on_update.call(props.nutrient_amount.new(value)),
            }
        } else {
            NutrientValue {
                symbol: props.nutrient_amount.symbol(),
                value: props.nutrient_amount.value(),
            }
        }
    }
}
