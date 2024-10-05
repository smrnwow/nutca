use crate::model::profiles::Profile;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfilesListingItemProps {
    profile: Profile,
    on_open: EventHandler<String>,
    on_use: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn ProfilesListingItem(props: ProfilesListingItemProps) -> Element {
    let profile = use_signal(|| props.profile);

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
                                props.on_open.call(profile.read().id().to_string());
                            },
                            "Открыть",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_use.call(profile.read().id().to_string());
                            },
                            "Рассчитать раствор",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_delete.call(profile.read().id().to_string());
                            },
                            "Удалить",
                        }
                    }
                }
            },

            Text {
                size: "x-small",
                {profile.read().name()},
            }
        }
    }
}
