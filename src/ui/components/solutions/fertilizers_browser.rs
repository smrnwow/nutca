use super::FertilizersBrowserItem;
use crate::model::fertilizers::FertilizersListing;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{Label, List, Pagination, Reference, Text, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserProps {
    fertilizers_listing: Signal<FertilizersListing>,
    on_select: EventHandler<String>,
    on_search: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn FertilizersBrowser(props: FertilizersBrowserProps) -> Element {
    let fertilizers_listing = use_memo(move || props.fertilizers_listing.read().clone());

    let mut show_reference = use_signal(|| false);

    rsx! {
        Column {
            gap: "medium",
            on_hover: move |hovered| show_reference.set(hovered),

            Row {
                Title {
                    size: "small",

                    "Выбор удобрений",

                    Reference {
                        display: show_reference,
                        style: "badge",
                        tooltip: rsx! {
                            Title {
                                size: "x-small",
                                "Пока не придуманый заголовок",
                            }

                            Text {
                                size: "x-small",
                                "Еще не придуманный текст. Еще не придуманный текст. Еще не придуманный текст.",
                            }
                        },
                        tooltip_position: "top-center",
                    },
                }
            }

            Label {
                text: "Поиск",

                TextField {
                    value: fertilizers_listing.read().search_query(),
                    placeholder: "название удобрения",
                    on_input: props.on_search,
                    icon_left: rsx! {
                        SearchIcon {}
                    }
                }
            }

            List {
                limit: 8,
                empty: fertilizers_listing.read().is_empty(),
                stub_text: "Удобрений не найдено",

                for fertilizer in fertilizers_listing.read().list() {
                    FertilizersBrowserItem {
                        key: "{fertilizer.id()}",
                        fertilizer,
                        on_select: props.on_select,
                    }
                }
            }

            Pagination {
                page_index: fertilizers_listing.read().page_index(),
                limit: fertilizers_listing.read().limit(),
                total: fertilizers_listing.read().total(),
                on_change: move |next_page| {
                    props.on_paginate.call(next_page);
                },
            }
        }
    }
}
