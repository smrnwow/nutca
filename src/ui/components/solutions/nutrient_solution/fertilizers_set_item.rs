use super::FertilizersSetTooltip;
use crate::model::fertilizers::FertilizerAmount;
use crate::ui::components::chemistry::SubstanceAmount;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::{FloatField, QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetItemProps {
    fertilizer_amount: Signal<FertilizerAmount>,
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
                    warn: props.fertilizer_amount.read().is_redurant(),
                    action_left: rsx! {
                        FertilizersSetTooltip {
                            fertilizer_amount: props.fertilizer_amount,
                        },
                    },
                    on_click: move |_| {
                        props.on_exclude.call(props.fertilizer_amount.read().fertilizer().id());
                    },

                    Text {
                        size: "x-small",
                        {props.fertilizer_amount.read().fertilizer().name()},
                    }
                }

                SubstanceAmount {
                    amount: props.fertilizer_amount.read().amount(),
                    is_liquid: props.fertilizer_amount.read().is_liquid(),
                    on_amount_update: move |value| {
                        props.on_amount_update.call((props.fertilizer_amount.read().fertilizer().id(), value));
                    },
                }

                /*
                div {
                    class: "fertilizers-set-item__amount",

                    FloatField {
                        value: props.fertilizer_amount.read().amount(),
                        on_change: move |value| {
                            props.on_amount_update.call((props.fertilizer_amount.read().fertilizer().id(), value));
                        },
                    }

                    span {
                        class: "fertilizers-set-item__units",
                        {props.fertilizer_amount.read().units()},
                    }
                }
                */
            }
        }
    }
}
