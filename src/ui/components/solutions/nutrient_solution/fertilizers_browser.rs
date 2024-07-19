use super::FertilizersBrowserItem;
use crate::repository::FertilizersListing;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{Label, List, Pagination, TextField, Title};
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
    let mut show_reference = use_signal(|| false);

    rsx! {
        Column {
            gap: "medium",
            on_hover: move |hovered| show_reference.set(hovered),

            Row {
                Title {
                    size: "small",
                    "Выбор удобрений",
                }
            }

            Label {
                text: "Поиск",

                TextField {
                    placeholder: "название удобрения",
                    value: props.fertilizers_listing.read().search_query(),
                    icon_left: rsx! {
                        SearchIcon {}
                    },
                    on_input: props.on_search,
                }
            }

            List {
                limit: 8,
                empty: props.fertilizers_listing.read().is_empty(),
                stub_text: "Удобрений не найдено",

                for fertilizer in props.fertilizers_listing.read().list() {
                    FertilizersBrowserItem {
                        key: "{fertilizer.id()}",
                        fertilizer: Signal::new(fertilizer),
                        on_select: props.on_select,
                    }
                }
            }

            Pagination {
                page_index: props.fertilizers_listing.read().page_index(),
                limit: props.fertilizers_listing.read().limit(),
                total: props.fertilizers_listing.read().total(),
                on_change: move |next_page| {
                    props.on_paginate.call(next_page);
                },
            }
        }
    }
}
