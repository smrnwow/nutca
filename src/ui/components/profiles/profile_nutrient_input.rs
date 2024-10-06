use crate::model::chemistry::NutrientAmount;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileNutrientInputProps {
    nutrient: NutrientAmount,
    on_update: Option<EventHandler<NutrientAmount>>,
}

#[component]
pub fn ProfileNutrientInput(props: ProfileNutrientInputProps) -> Element {
    rsx! {
        div {
            class: "profile-nutrient-input",

            if let Some(on_update) = props.on_update {
                NutrientValue {
                    symbol: props.nutrient.nutrient().symbol(),
                    value: Signal::new(props.nutrient.value()),
                    on_change: move |value| {
                        on_update.call(props.nutrient.new(value));
                    },
                }
            } else {
                NutrientValue {
                    symbol: props.nutrient.nutrient().symbol(),
                    value: Signal::new(props.nutrient.value()),
                }
            }
        }
    }
}
