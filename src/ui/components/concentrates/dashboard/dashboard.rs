use super::ListItem;
use crate::controller::concentrates::Listing;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{
    Block, Button, Card, Divider, List, Pagination, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DashboardProps {
    listing: Signal<Listing>,
    on_search: EventHandler<String>,
    on_add: EventHandler<()>,
    on_open: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    let concentrates = use_memo(move || props.listing.read().fetch());

    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        "Концентраты",
                    }
                }
            }

            Divider {}

            Block {
                Column {
                    gap: "medium",

                    Row {
                        gap: "medium",

                        TextField {
                            value: props.listing.read().search_query(),
                            placeholder: "найти концентрат",
                            on_input: props.on_search,
                            icon_left: rsx! {
                                SearchIcon {}
                            },
                        }

                        Button {
                            style: "primary",
                            on_click: props.on_add,
                            "Добавить концентрат",
                        }
                    }

                    List {
                        limit: 10,
                        empty: concentrates.len() == 0,
                        stub_text: "Сохраненные концентраты отсутствуют",

                        for concentrate in concentrates.read().iter() {
                            ListItem {
                                key: "{concentrate.id}",
                                concentrate: concentrate.clone(),
                                on_open: props.on_open,
                                on_delete: props.on_delete,
                            }
                        }
                    }

                    Pagination {
                        page_index: props.listing.read().page_index(),
                        limit: props.listing.read().limit(),
                        items_count: concentrates.read().len(),
                        on_change: props.on_paginate,
                    }
                }
            }
        }
    }
}
