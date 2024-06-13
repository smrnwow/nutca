use super::FertilizersBrowserItem;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{Search, Table, TableCell};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserProps {
    fertilizers: Memo<Vec<Fertilizer>>,
    selected_fertilizers: Memo<Vec<String>>,
    on_select: EventHandler<Fertilizer>,
    on_remove: EventHandler<String>,
    on_search: EventHandler<String>,
}

#[component]
pub fn FertilizersBrowser(props: FertilizersBrowserProps) -> Element {
    rsx! {
        div {
            class: "fertilizers-browser",

            div {
                class: "fertilizers-browser__search",

                Search {
                    placeholder: "найти удобрение",
                    on_change: props.on_search,
                }
            }

            div {
                class: "fertilizers-browser__table",

                Table {
                    header: rsx! {
                        TableCell {
                            width: "1%",
                        }

                        TableCell {
                            width: "50%",
                            "Название",
                        }

                        TableCell {
                            width: "50%",
                            "Состав",
                        }
                    },
                    body: rsx! {
                        for fertilizer in props.fertilizers.read().clone() {
                            FertilizersBrowserItem {
                                key: "{fertilizer.id()}",
                                selected: props.selected_fertilizers.read().contains(&fertilizer.id()),
                                fertilizer,
                                on_select: props.on_select,
                                on_remove: props.on_remove,
                            }
                        }
                    },
                }
            }
        }
    }
}
