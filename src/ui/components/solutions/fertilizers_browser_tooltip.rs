use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Tag, Text};
use dioxus::prelude::*;

fn tag_text(nutrient: Nutrient) -> Vec<String> {
    vec![
        nutrient.symbol().to_string(),
        format!("{:.1}%", nutrient.value()),
    ]
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserTooltipProps {
    fertilizer: Signal<Fertilizer>,
}

#[component]
pub fn FertilizersBrowserTooltip(props: FertilizersBrowserTooltipProps) -> Element {
    let macro_nutrients = props.fertilizer.read().macro_nutrients();

    let nitrogen_forms = props.fertilizer.read().nitrogen_forms();

    let micro_nutrients = props.fertilizer.read().micro_nutrients();

    rsx! {
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
    }
}
