use crate::controller::profiles::ProfileEditor;
use crate::controller::Toaster;
use crate::repository::Storage;
use crate::ui::components::profiles::ProfileEditor;
use dioxus::prelude::*;

#[component]
pub fn ProfileEditPage(profile_id: String) -> Element {
    let mut toaster = consume_context::<Signal<Toaster>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut profile_editor = use_signal(|| ProfileEditor::edit(storage, profile_id));

    let profile = profile_editor.read().profile();

    let nutrition_program = use_memo(move || profile_editor.read().nutrition_program().clone());

    let validation = profile_editor.read().validation();

    use_effect(move || toaster.write().render(validation.read().list()));

    rsx! {
        ProfileEditor {
            profile,
            nutrition_program,
            validation,
            on_nutrient_update: move |(stage_id, nutrient_amount)| {
                profile_editor.write().update_nutrient(stage_id, nutrient_amount);
            },
            on_name_update: move |name| {
                profile_editor.write().update_name(name);
            },
            on_stage_name_update: move |(stage_id, name)| {
                profile_editor.write().update_stage_name(stage_id, name);
            },
            on_stage_delete: move |stage_id| {
                profile_editor.write().remove_stage(stage_id);
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
