use crate::model::profiles::Profile;
use crate::ui::components::utils::icons::More;
use crate::ui::components::utils::{Button, Dropdown, DropdownOption, TableCell, TableRow};
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
        TableRow {
            TableCell {
                p {
                    class: "profiles-listing__name",
                    "{profile.read().name()}",
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
                                props.on_open.call(profile.read().id());
                                // println!("open profile {}", profile.read().id());
                            },

                            "Открыть",
                        }

                        DropdownOption {
                            on_click: move |_| {
                                props.on_use.call(profile.read().id());
                            },

                            "Использовать в растворе",
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
