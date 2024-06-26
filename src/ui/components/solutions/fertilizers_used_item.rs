use crate::model::chemistry::Nutrient;
use crate::model::solutions::FertilizerWeight;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::ArrowLeft;
use crate::ui::components::utils::{Tag, Text, Tooltip};
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

fn tag_text(nutrient: Nutrient) -> Vec<String> {
    vec![
        nutrient.symbol().to_string(),
        format!("{:.1}PPM", nutrient.value()),
    ]
}

fn item_class(is_redurant: bool) -> String {
    if is_redurant {
        String::from("fertilizers-used__item fertilizers-used__item_redurant")
    } else {
        String::from("fertilizers-used__item")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersUsedItemProps {
    fertilizer: FertilizerWeight,
    on_exclude: EventHandler<String>,
}

#[component]
pub fn FertilizersUsedItem(props: FertilizersUsedItemProps) -> Element {
    let macro_nutrients = props.fertilizer.macro_nutrients();

    let nitrogen_forms = props.fertilizer.nitrogen_forms();

    let micro_nutrients = props.fertilizer.micro_nutrients();

    let fertilizer = use_signal(|| props.fertilizer.fertilizer.clone());

    rsx! {
        Tooltip {
            position: "top-left",
            target: rsx! {
                div {
                    class: item_class(props.fertilizer.is_redurant()),
                    onclick: move |_| {
                        props.on_exclude.call(fertilizer.read().id());
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
                                "{fertilizer.read().name()}",
                            }
                        }

                        Text {
                            size: "x-small",
                            nowrap: true,
                            "{round(props.fertilizer.weight)} {units(fertilizer.read().liquid())}",
                        }
                    }
                }
            },
            body: rsx! {
                Column {
                    gap: "medium",

                    if props.fertilizer.is_redurant() {
                        Text {
                            size: "x-small",
                            "Удобрение не вносит питательных веществ в раствор и исключено из расчета",
                        }
                    }

                    Text {
                        size: "x-small",
                        nowrap: true,
                        bold: true,
                        "Исключить: {fertilizer.read().name()}",
                    }

                    Column {
                        gap: "small",

                        if macro_nutrients.len() > 0 {
                            Column {
                                gap: "xx-small",

                                Text {
                                    size: "xx-small",
                                    "Макроэлементы",
                                }

                                Row {
                                    gap: "x-small",
                                    for nutrient in macro_nutrients {
                                        Tag {
                                            multiple_text: tag_text(nutrient),
                                        }
                                    }
                                }
                            }
                        }

                        if nitrogen_forms.len() > 0 {
                            Column {
                                gap: "xx-small",

                                Text {
                                    size: "xx-small",
                                    "Формы азота",
                                }

                                Row {
                                    gap: "x-small",
                                    for nutrient in nitrogen_forms {
                                        Tag {
                                            multiple_text: tag_text(nutrient),
                                        }
                                    }
                                }
                            }
                        }

                        if micro_nutrients.len() > 0 {
                            Column {
                                gap: "xx-small",

                                Text {
                                    size: "xx-small",
                                    "Микроэлементы",
                                }

                                Row {
                                    gap: "x-small",
                                    for nutrient in micro_nutrients {
                                        Tag {
                                            multiple_text: tag_text(nutrient),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
