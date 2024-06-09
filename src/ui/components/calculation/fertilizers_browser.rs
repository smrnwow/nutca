use super::FertilizersBrowserItem;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::{Search, Table, TableCell};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserProps {
    fertilizers: Signal<Vec<(bool, Fertilizer)>>,
    on_select: EventHandler<(bool, String)>,
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
                        for (selected, fertilizer) in props.fertilizers.read().clone() {
                            FertilizersBrowserItem {
                                fertilizer: Signal::new(fertilizer),
                                selected,
                                on_select: props.on_select,
                            }
                        }
                    },
                }
            }
        }
    }
}
