use crate::controller::profiles::EditorFactory;
use crate::controller::Toaster;
use crate::repository::Storage;
use crate::ui::components::profiles::ProfileEditor;
use dioxus::prelude::*;

#[component]
pub fn ProfileEditPage(profile_id: String) -> Element {
    let toaster = consume_context::<Signal<Toaster>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut editor = use_signal(|| EditorFactory::new(storage, toaster).edit(&profile_id));

    let profile = use_memo(move || editor.read().profile().clone());

    let profile_validator = use_memo(move || editor.read().profile_validator().clone());

    rsx! {
        ProfileEditor {
            profile,
            profile_validator,
            on_name_update: move |name| {
                editor.write().update_name(name);
            },
            on_stage_add: move |_| {
                editor.write().add_stage();
            },
            on_stage_name_update: move |(stage_id, name)| {
                editor.write().update_stage_name(stage_id, name);
            },
            on_nutrient_update: move |(stage_id, nutrient_amount)| {
                editor.write().update_nutrient(stage_id, nutrient_amount);
            },
            on_stage_delete: move |stage_id| {
                editor.write().remove_stage(stage_id);
            },
            on_save: move |_| {
                editor.write().update();
            },
        }
    }
}
