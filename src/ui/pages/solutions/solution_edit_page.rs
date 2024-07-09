use crate::model::fertilizers::FertilizersListing;
use crate::model::profiles::ProfilesListing;
use crate::model::solutions::SolutionBuilder;
use crate::model::NotificationContainer;
use crate::storage::Storage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::solutions::SolutionEditor;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionEditPage(solution_id: String) -> Element {
    let mut notifications_container = consume_context::<Signal<NotificationContainer>>();

    let storage = consume_context::<Signal<Storage>>();

    let mut solution_builder = use_signal(|| {
        let solution = storage.read().solutions().get(solution_id);

        match solution {
            Ok(solution) => SolutionBuilder::from(solution),
            Err(_) => SolutionBuilder::new(),
        }
    });

    let solution = use_memo(move || solution_builder.read().build());

    let solution_error = use_memo(move || solution_builder.read().validate());

    let profile = use_memo(move || solution_builder.read().profile());

    let mut profiles_listing = use_signal(move || match storage.read().profiles().list() {
        Ok(listing) => listing,
        Err(_) => ProfilesListing::new(vec![]),
    });

    let profiles = use_memo(move || profiles_listing.read().list());

    let mut fertilizers_listing = use_signal(move || match storage.read().fertilizers().list() {
        Ok(listing) => listing,
        Err(_) => FertilizersListing::new(vec![]),
    });

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
                            storage.read().solutions().update(solution.read().clone()).unwrap();

                            navigator().push(Route::SolutionsListingPage {});
                        } else {
                            if let Some(error) = solution_error.read().name() {
                                notifications_container.write().add(error);
                            }
                        }
                    },
                }
            }
        }
    }
}
