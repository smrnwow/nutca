use crate::controller::profiles::ProfileEditor;
use crate::controller::Toaster;
use crate::repository::Storage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::profiles::ProfileEditor;
use dioxus::prelude::*;

#[component]
pub fn ProfileEditPage(profile_id: String) -> Element {
    let mut toaster = consume_context::<Signal<Toaster>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut profile_editor = use_signal(|| ProfileEditor::edit(storage, profile_id));

    let mut profile_builder = profile_editor.read().builder();

    let profile = profile_editor.read().profile();

    let validation = profile_editor.read().validation();

    use_effect(move || toaster.write().render(validation.read().list()));

    rsx! {
        Page {
            Section {
                ProfileEditor {
                    profile,
                    validation,
                    on_nutrient_update: move |nutrient_amount| {
                        profile_builder.write().update_nutrient(nutrient_amount);
                    },
                    on_name_update: move |name| {
                        profile_builder.write().update_name(name);
                    },
                    on_save: move |_| {
                        profile_editor.write().update();
                    },
                    on_cancel: move |_| {
                        profile_editor.write().back();
                    },
                }
            }
        }
    }
}
