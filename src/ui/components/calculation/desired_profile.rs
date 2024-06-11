use crate::model::profiles::{Component, Profile};
use crate::storage::ProfilesStorage;
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::utils::Select;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DesiredProfileProps {
    profile: Memo<Profile>,
    on_profile_change: EventHandler<Option<Profile>>,
    on_component_update: EventHandler<Component>,
}

#[component]
pub fn DesiredProfile(props: DesiredProfileProps) -> Element {
    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let profile = props.profile.read();

    let profiles = profiles_storage.read().list();

    let value = (profile.id(), profile.name());

    let options = use_signal(|| {
        let options: Vec<(String, String)> = profiles
            .iter()
            .map(|profile| (profile.id(), profile.name()))
            .collect();

        options
    });

    rsx! {
        div {
            class: "desired-profile",

            div {
                class: "desired_profile__browser",

                Select {
                    placeholder: "выбрать готовый профиль",
                    value,
                    options: options.read().clone(),
                    on_search: move |search_query| {
                        println!("on_search {}", search_query);
                    },
                    on_select: move |value| {
                        let profiles = profiles.clone();

                        let profile = profiles.iter().find(|profile| profile.id() == value);

                        match profile {
                            Some(profile) => {
                                println!("on_select {:#?}", profile);
                                props.on_profile_change.call(Some(profile.clone()));
                            },

                            None => {}
                        }
                    },
                    on_cancel: move |_| {
                        props.on_profile_change.call(None);
                    },
                }
            }

            ProfileForm {
                profile: props.profile,
                on_component_update: props.on_component_update,
            }
        }
    }
}
