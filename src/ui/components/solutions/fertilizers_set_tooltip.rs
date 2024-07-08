use crate::model::chemistry::Nutrient;
use crate::model::solutions::FertilizerWeight;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::ArrowLeft;
use crate::ui::components::utils::{Button, Tag, Text, Tooltip};
use dioxus::prelude::*;

fn tag_text(nutrient: Nutrient) -> Vec<String> {
    vec![
        nutrient.symbol().to_string(),
        format!("{:.1}PPM", nutrient.value()),
    ]
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetTooltipProps {
    fertilizer_weight: Signal<FertilizerWeight>,
}

#[component]
pub fn FertilizersSetTooltip(props: FertilizersSetTooltipProps) -> Element {
    let macro_nutrients = props.fertilizer_weight.read().macro_nutrients();

    let nitrogen_forms = props.fertilizer_weight.read().nitrogen_forms();

    let micro_nutrients = props.fertilizer_weight.read().micro_nutrients();

    rsx! {
        Tooltip {
            position: "top-left",
            target: rsx! {
                div {
                    class: "fertilizers-browser-tooltip",

                    Button {
                        style: "compact",
                        ArrowLeft {},
                    }

                    button {
                        class: "fertilizers-browser-tooltip__button",
                        "i",
                    }
                }
            },
            body: rsx! {
                Column {
                    gap: "medium",

                    if props.fertilizer_weight.read().is_redurant() {
                        Text {
                            size: "x-small",
                            "Удобрение не вносит питательных веществ в раствор и исключено из расчета",
                        }
                    }

                    Text {
                        size: "x-small",
                        bold: true,
                        "Исключить: {props.fertilizer_weight.read().fertilizer.name()}",
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
                                    wrap: true,
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
                                    wrap: true,
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
                                    wrap: true,
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
