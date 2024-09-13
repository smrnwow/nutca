use super::FertilizersBrowserItem;
use crate::controller::solutions::FertilizersPicker;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{Label, List, Pagination, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersBrowserProps {
    fertilizers_picker: Memo<FertilizersPicker>,
    on_select: EventHandler<String>,
    on_search: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn FertilizersBrowser(props: FertilizersBrowserProps) -> Element {
    let fertilizers = props.fertilizers_picker.read().browse();

    let items_count = fertilizers.len();

    rsx! {
        Column {
            gap: "medium",

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
                    value: props.fertilizers_picker.read().search_query(),
                    icon_left: rsx! {
                        SearchIcon {}
                    },
                    on_input: props.on_search,
                }
            }

            List {
                limit: 8,
                empty: items_count == 0,
                stub_text: "Удобрений не найдено",

                for fertilizer in fertilizers {
                    FertilizersBrowserItem {
                        key: "{fertilizer.id()}",
                        fertilizer: Signal::new(fertilizer),
                        on_select: props.on_select,
                    }
                }
            }

            Pagination {
                page_index: props.fertilizers_picker.read().page_index(),
                limit: props.fertilizers_picker.read().limit(),
                items_count,
                on_change: move |next_page| props.on_paginate.call(next_page),
            }
        }
    }
}
