use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizersListingItemProps {
    fertilizer: Fertilizer,
    on_open: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn FertilizersListingItem(props: FertilizersListingItemProps) -> Element {
    let fertilizer = use_signal(|| props.fertilizer);

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
            },

            Text {
                size: "x-small",
                {fertilizer.read().name()},
            }
        }
    }
}
