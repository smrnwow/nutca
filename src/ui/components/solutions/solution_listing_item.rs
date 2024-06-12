use crate::model::solutions::Solution;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, TableCell, TableRow};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionListingItemProps {
    solution: Solution,
    on_open: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn SolutionListingItem(props: SolutionListingItemProps) -> Element {
    let solution = use_signal(|| props.solution);

    rsx! {
        TableRow {
            TableCell {
                p {
                    class: "solutions-listing__name",
                    "{solution.read().name()}",
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
                                props.on_open.call(solution.read().id());
                            },

                            "Открыть",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_delete.call(solution.read().id());
                            },

                            "Удалить",
                        }
                    }
                }
            }
        }
    }
}
