use super::FertilizersSetTooltip;
use crate::model::solutions::FertilizerWeight;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::{FloatField, QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetItemProps {
    fertilizer_weight: Signal<FertilizerWeight>,
    on_exclude: EventHandler<String>,
    on_amount_update: EventHandler<(String, f64)>,
}

#[component]
pub fn FertilizersSetItem(props: FertilizersSetItemProps) -> Element {
    rsx! {
        div {
            class: "fertilizers-set-item",

            Row {
                gap: "x-small",

                QuickAction {
                    warn: props.fertilizer_weight.read().is_redurant(),
                    action_left: rsx! {
                        FertilizersSetTooltip {
                            fertilizer_weight: props.fertilizer_weight,
                        },
                    },
                    on_click: move |_| {
                        props.on_exclude.call(props.fertilizer_weight.read().id());
                    },

                    Text {
                        size: "x-small",
                        {props.fertilizer_weight.read().name()},
                    }
                }

                div {
                    class: "fertilizers-set-item__amount",

                    FloatField {
                        value: props.fertilizer_weight.read().weight(),
                        on_change: move |value| {
                            props.on_amount_update.call((props.fertilizer_weight.read().id(), value));
                        },
                    }
                }
            }
        }
    }
}
