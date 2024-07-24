use crate::ui::components::NutrientValue;
use dioxus::prelude::*;
use nutca::fertilizers::labels::Component;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerComponentInputProps {
    component: Component,
    on_update: EventHandler<Component>,
}

#[component]
pub fn FertilizerComponentInput(props: FertilizerComponentInputProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.component.symbol(),
            value: props.component.value(),
            on_change: move |value| props.on_update.call(props.component.new(value)),
        }
    }
}
