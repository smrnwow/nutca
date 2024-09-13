use crate::controller::solutions::EditorFactory;
use crate::controller::Toaster;
use crate::repository::Storage;
use crate::ui::components::solutions::SolutionEditor;
use dioxus::prelude::*;

#[component]
pub fn SolutionAddPage(profile_id: String) -> Element {
    let toaster = consume_context::<Signal<Toaster>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut editor = use_signal(|| EditorFactory::new(storage, toaster).create(profile_id));

    let nutrition_program_browser =
        use_memo(move || editor.read().nutrition_program_browser().clone());

    let fertilizers_picker = use_memo(move || editor.read().fertilizers_picker().clone());

    let solution = use_memo(move || editor.read().solution().clone());

    let validator = use_memo(move || editor.read().validator().clone());

    let fertilizers_used = use_memo(move || editor.read().fertilizers_used().clone());

    rsx! {
        SolutionEditor {
            nutrition_program_browser,
            fertilizers_picker,
            solution,
            validator,
            fertilizers_used,
            on_profile_change: move |profile_id| {
                editor.write().change_nutrition_program(profile_id);
            },
            on_fertilizer_select: move |fertilizer_id| {
                editor.write().select_fertilizer(fertilizer_id);
            },
            on_fertilizer_exclude: move |fertilizer_id| {
                editor.write().exclude_fertilizer(fertilizer_id);
            },
            on_fertilizer_amount_update: move |(fertilizer_id, amount)| {
                editor.write().update_fertilizer_amount(fertilizer_id, amount);
            },
            on_name_update: move |name| {
                editor.write().update_name(name);
            },
            on_profile_nutrient_update: move |nutrient_requirement| {
                editor.write().update_nutrient_requirement(nutrient_requirement);
            },
            on_volume_update: move |volume| {
                editor.write().update_volume(volume);
            },
            on_profile_search: move |search_query| {
                editor.write().search_nutrient_program(search_query);
            },
            on_fertilizer_search: move |search_query| {
                editor.write().search_fertilizer(search_query);
            },
            on_fertilizers_paginate: move |page_index| {
                editor.write().paginate_fertilizers_browser(page_index);
            },
            on_selected_set_paginate: move |page_index| {
                editor.write().paginate_selected_set(page_index);
            },
            on_save: move |_| {
                editor.write().create();
            },
        }
    }
}
