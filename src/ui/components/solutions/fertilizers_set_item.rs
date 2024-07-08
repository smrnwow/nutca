use crate::model::solutions::FertilizerWeight;
use crate::ui::components::solutions::FertilizersSetTooltip;
use crate::ui::components::utils::{QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetItemProps {
    fertilizer_weight: Signal<FertilizerWeight>,
    on_exclude: EventHandler<String>,
}

#[component]
pub fn FertilizersSetItem(props: FertilizersSetItemProps) -> Element {
    rsx! {
        QuickAction {
            warn: props.fertilizer_weight.read().is_redurant(),
            action_left: rsx! {
                FertilizersSetTooltip {
                    fertilizer_weight: props.fertilizer_weight,
                },
            },
            on_click: move |_| {
                props.on_exclude.call(props.fertilizer_weight.read().fertilizer.id());
            },

            Text {
                size: "x-small",
                {props.fertilizer_weight.read().fertilizer.name()},
            }

            Text {
                size: "x-small",
                nowrap: true,
                {props.fertilizer_weight.read().display_amount()},
            }
        }
    }
}
