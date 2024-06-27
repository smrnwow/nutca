use crate::model::profiles::ProfileBuilder;
use crate::storage::ProfilesStorage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::profiles::ProfileEditor;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProfileEditPage(profile_id: String) -> Element {
    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let mut profile_builder = use_signal(|| {
        let profile = profiles_storage.read().get(profile_id);

        match profile {
            Some(profile) => ProfileBuilder::from(profile),
            None => ProfileBuilder::new(),
        }
    });

    let profile = use_memo(move || profile_builder.read().build());

    rsx! {
        Page {
            Section {
                ProfileEditor {
                    profile,
                    on_nutrient_update: move |nutrient| {
                        profile_builder.write().update_nutrient(nutrient);
                    },
                    on_name_update: move |name| {
                        profile_builder.write().update_name(name);
                    },
                    on_save: move |_| {
                        let storage = profiles_storage.read();

                        storage.update(profile.read().clone());

                        navigator().push(Route::ProfilesListingPage {});
                    },
                    on_cancel: move |_| {
                        navigator().push(Route::ProfilesListingPage {});
                    },
                }
            }
        }
    }
}
