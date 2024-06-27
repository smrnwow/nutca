use crate::model::fertilizers::Fertilizer;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerListingItemProps {
    fertilizer: Fertilizer,
    on_open: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn FertilizerListingItem(props: FertilizerListingItemProps) -> Element {
    let fertilizer = use_signal(|| props.fertilizer);

    rsx! {
        div {
            class: "fertilizers-listing-item",

            Row {
                horizontal: "space-between",
                vertical: "center",

                Text {
                    size: "x-small",
                    "{fertilizer.read().name()}",
                }

                Dropdown {
                    header: rsx! {
                        Button {
                            style: "compact",

                            More {}
                        }
                    },

                    options: rsx! {
                        DropdownOption {
                            on_click: move |_| {
                                props.on_open.call(fertilizer.read().id());
                            },

                            "Открыть",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_delete.call(fertilizer.read().id());
                            },

                            "Удалить",
                        }
                    }
                }
            }
        }
    }
}
