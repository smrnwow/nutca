use crate::model::solutions::FertilizerWeight;
use crate::ui::components::solutions::FertilizersSetTooltip;
use crate::ui::components::utils::icons::ArrowLeft;
use crate::ui::components::utils::{QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetItemProps {
    fertilizer: FertilizerWeight,
    on_exclude: EventHandler<String>,
}

#[component]
pub fn FertilizersSetItem(props: FertilizersSetItemProps) -> Element {
    let fertilizer_weight = use_signal(|| props.fertilizer);

    rsx! {
        QuickAction {
            warn: fertilizer_weight.read().is_redurant(),
            action_left: rsx! {
                ArrowLeft {},
            },
            reference: rsx! {
                FertilizersSetTooltip {
                    fertilizer_weight,
                },
            },
            on_click: move |_| {
                props.on_exclude.call(fertilizer_weight.read().fertilizer.id());
            },

            Text {
                size: "x-small",
                {fertilizer_weight.read().fertilizer.name()},
            }

            Text {
                size: "x-small",
                nowrap: true,
                {fertilizer_weight.read().display_amount()},
            }
        }
    }
}
