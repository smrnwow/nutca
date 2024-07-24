use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, QuickAction, Text};
use dioxus::prelude::*;
use nutca::solutions::Solution;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionListingItemProps {
    solution: Solution,
    on_open: EventHandler<String>,
    on_stock: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn SolutionsListingItem(props: SolutionListingItemProps) -> Element {
    let solution = use_signal(|| props.solution);

    rsx! {
        QuickAction {
            action_right: rsx! {
                Dropdown {
                    header: rsx! {
                        Button {
                            style: "compact",
                            More {},
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
                                props.on_stock.call(solution.read().id());
                            },
                            "Рассчитать рабочий раствор",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_delete.call(solution.read().id());
                            },
                            "Удалить",
                        }
                    }
                }
            },

            Text {
                size: "x-small",
                {solution.read().name()},
            }
        }
    }
}
