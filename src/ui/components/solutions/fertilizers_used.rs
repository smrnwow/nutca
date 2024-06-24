use crate::model::solutions::{FertilizerWeight, Solution};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::ArrowLeft;
use crate::ui::components::utils::{Divider, NumberField, Text, Title};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

fn units(liquid: bool) -> String {
    match liquid {
        true => String::from("мл"),
        false => String::from("г"),
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersUsedProps {
    fertilizers_used: Memo<Vec<FertilizerWeight>>,
    solution: Memo<Solution>,
    on_volume_update: EventHandler<usize>,
    on_remove: EventHandler<String>,
}

#[component]
pub fn FertilizersUsed(props: FertilizersUsedProps) -> Element {
    rsx! {
        Column {
            Title {
                size: "small",
                text: "Используемые удобрения",
            }

            Row {
                gap: "small",
                vertical: "center",

                Text {
                    nowrap: true,

                    "Объем раствора: ",
                }

                NumberField {
                    value: props.solution.read().water_amount(),
                    units: "литр",
                    on_change: props.on_volume_update,
                }
            }

            if props.fertilizers_used.read().len() == 0 {
                div {
                    class: "fertilizers-used__stub",

                    Text {
                        size: "x-small",
                        "Выберите удобрения из списка",
                    }
                }
            } else {
                div {
                    class: "fertilizers-used__table",

                    for fertilizer in props.fertilizers_used.read().clone() {
                        div {
                            class: "fertilizers-used__item",
                            key: "{fertilizer.fertilizer.id()}",
                            onclick: move |_| {
                                props.on_remove.call(fertilizer.fertilizer.id());
                            },

                            Row {
                                horizontal: "space-between",
                                vertical: "center",

                                Row {
                                    gap: "x-small",
                                    vertical: "center",

                                    ArrowLeft {}

                                    Text {
                                        size: "x-small",
                                        "{fertilizer.fertilizer.name()}",
                                    }
                                }

                                Text {
                                    size: "x-small",
                                    nowrap: true,
                                    "{round(fertilizer.weight)} {units(fertilizer.fertilizer.liquid())}",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
