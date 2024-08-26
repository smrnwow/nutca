use crate::controller::solutions::SolutionEditor;
use crate::controller::Toaster;
use crate::repository::Storage;
use crate::ui::components::solutions::SolutionEditor;
use dioxus::prelude::*;

#[component]
pub fn SolutionEditPage(solution_id: String) -> Element {
    let mut toaster = consume_context::<Signal<Toaster>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut solution_editor = use_signal(|| SolutionEditor::edit(storage, solution_id));

    let mut profiles_listing = solution_editor.read().profiles_listing();

    let mut fertilizers_listing = solution_editor.read().fertilizers_listing();

    let mut solution_builder = solution_editor.read().builder();

    let solution = solution_editor.read().solution();

    let validation = solution_editor.read().validation();

    let profile = solution_editor.read().profile();

    let build_mode = use_memo(move || solution_builder.read().mode());

    use_effect(move || toaster.write().render(validation.read().list()));

    rsx! {
        SolutionEditor {
            profiles_listing,
            fertilizers_listing,
            validation,
            solution,
            profile,
            build_mode,
            on_profile_change: move |profile_id| {
                solution_editor.write().change_profile(profile_id);
            },
            on_fertilizer_select: move |fertilizer_id| {
                solution_editor.write().select_fertilizer(fertilizer_id);
            },
            on_fertilizer_exclude: move |fertilizer_id| {
                solution_editor.write().exclude_fertilizer(fertilizer_id);
            },
            on_fertilizer_amount_update: move |(fertilizer_id, amount)| {
                solution_builder.write().update_fertilizer_amount(fertilizer_id, amount);
            },
            on_name_update: move |name| {
                solution_builder.write().name(name);
            },
            on_volume_update: move |volume| {
                solution_builder.write().volume(volume);
            },
            on_profile_nutrient_update: move |nutrient| {
                solution_builder.write().nutrient_requirement(nutrient);
            },
            on_profile_search: move |search_query| {
                profiles_listing.write().search(search_query);
            },
            on_fertilizer_search: move |search_query| {
                fertilizers_listing.write().search(search_query);
            },
            on_fertilizers_paginate: move |page_index| {
                fertilizers_listing.write().paginate(page_index);
            },
            on_save: move |_| {
                solution_editor.write().update();
            },
        }
    }
}
