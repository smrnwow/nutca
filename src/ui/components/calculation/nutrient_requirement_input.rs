use crate::model::calculation::NutrientRequirement;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NutrientRequirementInputProps {
    nutrient_requirement: NutrientRequirement,
    on_update: EventHandler<NutrientRequirement>,
}

#[component]
pub fn NutrientRequirementInput(props: NutrientRequirementInputProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.nutrient_requirement.symbol(),
            value: props.nutrient_requirement.amount(),
            on_change: move |value| props.on_update.call(props.nutrient_requirement.new(value)),
        }
    }
}
