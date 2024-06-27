use crate::model::profiles::Profile;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileListingItemProps {
    profile: Profile,
    on_open: EventHandler<String>,
    on_use: EventHandler<String>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn ProfileListingItem(props: ProfileListingItemProps) -> Element {
    let profile = use_signal(|| props.profile);

    rsx! {
        div {
            class: "profiles-listing-item",

            Row {
                horizontal: "space-between",
                vertical: "center",

                Text {
                    size: "x-small",
                    {profile.read().name()},
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
                                props.on_open.call(profile.read().id());
                            },

                            "Открыть",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_use.call(profile.read().id());
                            },

                            "Рассчитать раствор",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_delete.call(profile.read().id());
                            },

                            "Удалить",
                        }
                    }
                }
            }
        }
    }
}
