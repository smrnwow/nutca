use crate::controller::concentrates::EditorFactory;
use crate::repository::Storage;
use crate::ui::components::concentrates::editor::ConcentrateEditor;
use dioxus::prelude::*;

#[component]
pub fn Create(solution_id: String) -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut editor = use_signal(|| EditorFactory::new(storage).create(solution_id));

    let concentrate = use_memo(move || editor.read().concentrate().clone());

    let concentrate_composition = use_memo(move || editor.read().concentrate_composition().clone());

    let solutions_browser = use_memo(move || editor.read().solutions_browser().clone());

    let fertilizers_browser = use_memo(move || editor.read().fertilizers_browser().clone());

    rsx! {
        ConcentrateEditor {
            concentrate,
            concentrate_composition,
            solutions_browser,
            fertilizers_browser,
            on_solution_search: move |search_query| {
                editor.write().search_solution(search_query);
            },
            on_solution_change: move |solution_id| {
                editor.write().change_solution(solution_id);
            },
            on_composition_type_change: move |composition_type| {
                editor.write().change_composition_type(composition_type);
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
            on_fertilizer_percent_distribute: move |(part_id, fertilizer_id, percent)| {
                editor.write().update_fertilizer_percent(part_id, fertilizer_id, percent);
            },
            on_fertilizer_amount_add: move |(part_id, fertilizer_id, percent)| {
                editor.write().update_fertilizer_amount(part_id, fertilizer_id, percent);
            },
            on_fertilizer_delete: move |(part_id, fertilizer_id)| {
                editor.write().remove_fertilizer(part_id, fertilizer_id);
            },
            on_save: move |_| {
                editor.write().create_concentrate();
            }
        }
    }
}
