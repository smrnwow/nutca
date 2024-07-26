use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::ArrowLeft;
use crate::ui::components::utils::{Button, Tag, Text, Tooltip};
use dioxus::prelude::*;
use nutca::chemistry::NutrientAmount;
use nutca::solutions::FertilizerWeight;

fn tag_text(nutrient: NutrientAmount) -> Vec<String> {
    vec![
        nutrient.nutrient().symbol().to_string(),
        format!("{:.1}PPM", nutrient.value() / 10.),
    ]
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersSetTooltipProps {
    fertilizer_weight: Signal<FertilizerWeight>,
}

#[component]
pub fn FertilizersSetTooltip(props: FertilizersSetTooltipProps) -> Element {
    let macros = props.fertilizer_weight.read().nutrients().macros();

    let nitrogen_forms = props.fertilizer_weight.read().nutrients().nitrogen_forms();

    let micros = props.fertilizer_weight.read().nutrients().micros();

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
                        "Исключить: {props.fertilizer_weight.read().name()}",
                    }

                    Column {
                        gap: "small",

                        if macros.len() > 0 {
                            Column {
                                gap: "xx-small",

                                Text {
                                    size: "xx-small",
                                    "Макроэлементы",
                                }

                                Row {
                                    gap: "x-small",
                                    wrap: true,
                                    for nutrient in macros {
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

                        if micros.len() > 0 {
                            Column {
                                gap: "xx-small",

                                Text {
                                    size: "xx-small",
                                    "Микроэлементы",
                                }

                                Row {
                                    gap: "x-small",
                                    wrap: true,
                                    for nutrient in micros {
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
