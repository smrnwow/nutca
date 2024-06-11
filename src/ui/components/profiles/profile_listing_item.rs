use crate::model::profiles::Profile;
use crate::ui::components::utils::{TableCell, TableRow};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileListingItemProps {
    profile: Profile,
    on_select: EventHandler<String>,
}

#[component]
pub fn ProfileListingItem(props: ProfileListingItemProps) -> Element {
    let profile_id = props.profile.id();

    let profile_name = props.profile.name();

    rsx! {
        TableRow {
            on_click: move |_| {
                props.on_select.call(profile_id.clone());
            },

            TableCell {
                p {
                    class: "profiles-listing__name",
                    "{profile_name}",
                }
            }
        }
    }
}
