use super::FertilizersBrowserItem;
use crate::model::fertilizers::FertilizersListing;
use crate::ui::components::layout::Column;
use crate::ui::components::utils::{Pagination, Search, Title};
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

    rsx! {
        Column {
            Title {
                size: "small",
                text: "Выбор удобрений",
            }

            Search {
                placeholder: "найти удобрение",
                on_change: props.on_search,
            }

            div {
                class: "fertilizers-browser__table",

                for fertilizer in fertilizers_listing.read().list() {
                    FertilizersBrowserItem {
                        key: "{fertilizer.id()}",
                        fertilizer,
                        on_select: props.on_select,
                    }
                }
            }

            div {
                class: "fertilizers-browser__pagination",

                Pagination {
                    page_index: fertilizers_listing.read().page_index(),
                    limit: 8,
                    total: fertilizers_listing.read().total(),
                    on_change: move |next_page| {
                        props.on_paginate.call(next_page);
                    },
                }
            }
        }
    }
}
