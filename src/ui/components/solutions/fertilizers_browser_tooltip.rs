use crate::model::chemistry::NutrientAmount;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::ArrowRight;
use crate::ui::components::utils::{Button, Tag, Text, Tooltip};
use dioxus::prelude::*;

fn tag_text(nutrient: NutrientAmount) -> Vec<String> {
    vec![
        nutrient.nutrient().symbol().to_string(),
        format!("{:.1}%", nutrient.value()),
    ]
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserTooltipProps {
    fertilizer: Signal<Fertilizer>,
}

#[component]
pub fn FertilizersBrowserTooltip(props: FertilizersBrowserTooltipProps) -> Element {
    let macro_nutrients = props.fertilizer.read().nutrients.macros();

    let nitrogen_forms = props.fertilizer.read().nutrients.nitrogen_forms();

    let micro_nutrients = props.fertilizer.read().nutrients.micros();

    rsx! {
        Tooltip {
            position: "top-right",
            target: rsx! {
                div {
                    class: "fertilizers-browser-tooltip",

                    Button {
                        style: "compact",
                        ArrowRight {},
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

                    Text {
                        size: "x-small",
                        bold: true,
                        "Использовать: {props.fertilizer.read().name()}",
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
            },
        }
    }
}
