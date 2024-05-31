use crate::model::labels::Component;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersComponentProps {
    component: Component,
    on_update: EventHandler<Component>,
}

#[component]
pub fn FertilizersComponent(props: FertilizersComponentProps) -> Element {
    rsx! {
        NutrientValue {
            symbol: props.component.symbol(),
            value: props.component.value(),
            on_change: move |value| props.on_update.call(props.component.new(value)),
        }
    }
}
