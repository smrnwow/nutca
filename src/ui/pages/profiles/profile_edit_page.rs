use crate::model::profiles::ProfileBuilder;
use crate::storage::Storage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::profiles::ProfileEditor;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProfileEditPage(profile_id: String) -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut profile_builder = use_signal(|| {
        let profile = storage.read().profiles().get(profile_id);

        match profile {
            Ok(profile) => ProfileBuilder::from(profile),
            Err(_) => ProfileBuilder::new(),
        }
    });

    let profile = use_memo(move || profile_builder.read().build());

    let profile_error = use_memo(move || profile_builder.read().validate());

    rsx! {
        Page {
            Section {
                ProfileEditor {
                    profile,
                    profile_error,
                    on_nutrient_update: move |nutrient| {
                        profile_builder.write().update_nutrient(nutrient);
                    },
                    on_name_update: move |name| {
                        profile_builder.write().update_name(name);
                    },
                    on_save: move |_| {
                        profile_builder.write().save();

                        if profile_error.read().is_empty() {
                            storage.read().profiles().update(profile.read().clone()).unwrap();

                            navigator().push(Route::ProfilesListingPage {});
                        }
                    },
                    on_cancel: move |_| {
                        navigator().push(Route::ProfilesListingPage {});
                    },
                }
            }
        }
    }
}
