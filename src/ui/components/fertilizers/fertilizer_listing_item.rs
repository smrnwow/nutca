use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, TableCell, TableRow, Tag};
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
        TableRow {
            TableCell {
                p {
                    class: "fertilizer-listing__name",
                    "{fertilizer.read().name()}",
                }

                p {
                    class: "fertilizer-listing__vendor",
                    "Производитель: {fertilizer.read().vendor()}",
                }
            }

            TableCell {
                div {
                    class: "fertilizer-listing__nutrients",

                    for nutrient in fertilizer.read().nutrients() {
                        Tag {
                            text: nutrient.symbol(),
                        }
                    }
                }
            }

            TableCell {
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
