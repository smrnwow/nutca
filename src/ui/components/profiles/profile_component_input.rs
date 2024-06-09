use crate::model::profiles::Component;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileComponentInputProps {
    component: Component,
    on_update: Option<EventHandler<Component>>,
}

#[component]
pub fn ProfileComponentInput(props: ProfileComponentInputProps) -> Element {
    match props.component {
        Component::Nutrient(nutrient_amount) => {
            rsx! {
                if props.on_update.is_some() {
                    NutrientValue {
                        symbol: nutrient_amount.symbol(),
                        value: nutrient_amount.value(),
                        on_change: move |value| {
                            props.on_update.unwrap().call(Component::Nutrient(nutrient_amount.new(value)));
                        },
                    }
                } else {
                    NutrientValue {
                        symbol: nutrient_amount.symbol(),
                        value: nutrient_amount.value(),
                    }
                }
            }
        }

        Component::NitrogenForm(nitrogen_form) => {
            rsx! {
                if props.on_update.is_some() {
                    NutrientValue {
                        symbol: nitrogen_form.symbol(),
                        value: nitrogen_form.value(),
                        on_change: move |value| {
                            props.on_update.unwrap().call(Component::NitrogenForm(nitrogen_form.new(value)));
                        },
                    }
                } else {
                    NutrientValue {
                        symbol: nitrogen_form.symbol(),
                        value: nitrogen_form.value(),
                    }
                }
            }
        }
    }
}
