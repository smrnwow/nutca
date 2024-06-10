use crate::model::profiles::Profile;
use crate::storage::ProfilesStorage;
use crate::ui::components::profiles::ProfileEditorWorkspace;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProfileEditorPage() -> Element {
    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let mut profile = use_signal(|| Profile::new());

    rsx! {
        div {
            class: "profile-editor-page",

            section {
                class: "profile-editor-page__workspace",

                ProfileEditorWorkspace {
                    profile,
                    on_component_update: move |component| {
                        profile.write().set_component(component);
                    },
                    on_name_update: move |name| {
                        profile.write().set_name(name);
                    },
                    on_save: move |_| {
                        let storage = profiles_storage.read();

                        profile.write().create_id();

                        storage.add(profile.read().clone());

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
