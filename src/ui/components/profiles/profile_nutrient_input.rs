use crate::model::chemistry::Nutrient;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileNutrientInputProps {
    nutrient: Nutrient,
    on_update: Option<EventHandler<Nutrient>>,
}

#[component]
pub fn ProfileNutrientInput(props: ProfileNutrientInputProps) -> Element {
    rsx! {
        if let Some(on_update) = props.on_update {
            NutrientValue {
                symbol: props.nutrient.symbol(),
                value: props.nutrient.value(),
                on_change: move |value| {
                    on_update.call(props.nutrient.new(value));
                },
            }
        } else {
            NutrientValue {
                symbol: props.nutrient.symbol(),
                value: props.nutrient.value(),
            }
        }
    }
}
