use crate::model::concentrates::Concentrate;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ListItemProps {
    concentrate: Concentrate,
    on_open: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn ListItem(props: ListItemProps) -> Element {
    let concentrate = use_signal(|| props.concentrate);

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
                                props.on_open.call(concentrate.read().id().clone());
                            },
                            "Открыть",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_delete.call(concentrate.read().id().clone());
                            },
                            "Удалить",
                        }
                    }
                }
            },

            Text {
                size: "x-small",
                {concentrate.read().name().clone()},
            }
        }
    }
}
