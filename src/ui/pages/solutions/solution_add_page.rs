use crate::model::solutions::SolutionBuilder;
use crate::storage::{FertilizersStorage, ProfilesStorage, SolutionsStorage};
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::solutions::SolutionEditor;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionAddPage(profile_id: String) -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut solution_builder = use_signal(|| {
        let profile = profiles_storage.read().get(profile_id);

        match profile {
            Some(profile) => SolutionBuilder::base_on(profile),
            None => SolutionBuilder::new(),
        }
    });

    let solution = use_memo(move || solution_builder.read().build());

    let solution_error = use_memo(move || solution_builder.read().validate());

    let profile = use_memo(move || solution_builder.read().profile());

    let mut profiles_listing = use_signal(move || profiles_storage.read().list());

    let profiles = use_memo(move || profiles_listing.read().list());

    let mut fertilizers_listing = use_signal(move || fertilizers_storage.read().list());

    rsx! {
        Page {
            Section {
                SolutionEditor {
                    solution,
                    solution_error,
                    fertilizers_listing,
                    profile,
                    profiles,
                    on_name_update: move |name| {
                        solution_builder.write().update_name(name);
                    },
                    on_volume_update: move |volume| {
                        solution_builder.write().update_volume(volume);
                    },
                    on_profile_nutrient_update: move |nutrient| {
                        solution_builder.write().update_profile_nutrient(nutrient);
                    },
                    on_profile_change: move |profile_id: String| {
                        let profile = profiles_listing.read().find(profile_id);

                        solution_builder.write().update_profile(profile);
                    },
                    on_profile_search: move |search_query| {
                        profiles_listing.write().search(search_query);
                    },
                    on_fertilizer_select: move |fertilizer_id| {
                        if let Some(fertilizer) = fertilizers_listing.write().exclude(fertilizer_id) {
                            solution_builder.write().add_fertilizer(fertilizer);
                        }
                    },
                    on_fertilizer_exclude: move |fertilizer_id: String| {
                        fertilizers_listing.write().include(fertilizer_id.clone());

                        solution_builder.write().remove_fertilizer(fertilizer_id);
                    },
                    on_fertilizer_search: move |search_query| {
                        fertilizers_listing.write().search(search_query);
                    },
                    on_fertilizers_paginate: move |page_index| {
                        fertilizers_listing.write().paginate(page_index);
                    },
                    on_save: move |_| {
                        solution_builder.write().save();

                        if solution_error.read().is_empty() {
                            let solution = solution.read().clone();

                            solutions_storage.read().add(solution);

                            navigator().push(Route::SolutionsListingPage {});
                        }
                    },
                }
            }
        }
    }
}
