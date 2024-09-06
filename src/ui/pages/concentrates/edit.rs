use crate::controller::concentrates::Editor;
use crate::repository::Storage;
use crate::ui::components::concentrates::editor::ConcentrateEditor;
use dioxus::prelude::*;

#[component]
pub fn Edit(concentrate_id: String) -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut editor = use_signal(|| Editor::from_concentrate(storage, concentrate_id));

    let solutions_browser = use_memo(move || editor.read().solutions_browser().clone());

    let tanks_set = use_memo(move || editor.read().tanks_set().clone());

    let concentrate = use_memo(move || editor.read().concentrate());

    rsx! {
        ConcentrateEditor {
            concentrate,
            tanks_set,
            solutions_browser,
            on_solution_search: move |search_query| {
                editor.write().search_solution(search_query);
            },
            on_solution_change: move |solution_id| {
                editor.write().change_solution(solution_id);
            },
            on_filler_variant_change: move |filler_variant| {
                editor.write().change_filler_variant(filler_variant);
            },
            on_part_add: move |_| {
                editor.write().add_part();
            },
            on_part_delete: move |part_id| {
                editor.write().delete_part(part_id);
            },
            on_name_update: move |name| {
                editor.write().update_name(name);
            },
            on_part_name_update: move |(part_id, name)| {
                editor.write().update_part_name(part_id, name);
            },
            on_part_concentration_update: move |(part_id, concentration)| {
                editor.write().update_part_concentration(part_id, concentration);
            },
            on_part_volume_update: move |(part_id, volume)| {
                editor.write().update_part_volume(part_id, volume);
            },
            on_part_fertilizer_add: move |(part_id, fertilizer_id, percent)| {
                editor.write().add_part_fertilizer(part_id, fertilizer_id, percent);
            },
            on_part_fertilizer_delete: move |(part_id, fertilizer_id)| {
                editor.write().delete_part_fertilizer(part_id, fertilizer_id);
            },
            on_save: move |_| {
                editor.write().edit_concentrate();
            }
        }
    }
}
