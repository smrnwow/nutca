use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::ArrowRight;
use crate::ui::components::utils::{Tag, Text, Tooltip};
use dioxus::prelude::*;

fn tag_text(nutrient: Nutrient) -> Vec<String> {
    vec![
        nutrient.symbol().to_string(),
        format!("{:.1}%", nutrient.value()),
    ]
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserItemProps {
    fertilizer: Fertilizer,
    on_select: EventHandler<String>,
}

#[component]
pub fn FertilizersBrowserItem(props: FertilizersBrowserItemProps) -> Element {
    let macro_nutrients = props.fertilizer.macro_nutrients();

    let nitrogen_forms = props.fertilizer.nitrogen_forms();

    let micro_nutrients = props.fertilizer.micro_nutrients();

    let fertilizer = use_signal(|| props.fertilizer);

    rsx! {
        Tooltip {
            position: "top-right",

            target: rsx! {
                div {
                    class: "fertilizers-browser__item",
                    onclick: move |_| {
                        props.on_select.call(fertilizer.read().id());
                    },

                    Row {
                        gap: "medium",
                        horizontal: "space-between",
                        vertical: "center",

                        Text {
                            size: "x-small",
                            {fertilizer.read().name()},
                        }

                        ArrowRight {}
                    }
                }
            },
            body: rsx! {
                Column {
                    gap: "medium",

                    Text {
                        size: "x-small",
                        nowrap: true,
                        bold: true,
                        "Использовать: {fertilizer.read().name()}",
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
                                    for nutrient in fertilizer.read().macro_nutrients() {
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
                                    for nutrient in fertilizer.read().nitrogen_forms() {
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
                                    for nutrient in fertilizer.read().micro_nutrients() {
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
