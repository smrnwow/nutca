use crate::model::fertilizers::FertilizersListing;
use crate::model::profiles::ProfilesListing;
use crate::model::solutions::SolutionBuilder;
use crate::storage::{FertilizersStorage, ProfilesStorage, SolutionsStorage};
use crate::ui::components::solutions::{SolutionEditor, SolutionPreview};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionEditPage(solution_id: String) -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let mut solution_builder = use_signal(|| {
        let solution = solutions_storage.read().get(solution_id);

        match solution {
            Some(solution) => SolutionBuilder::from(solution),
            None => SolutionBuilder::new(),
        }
    });

    let solution = use_memo(move || solution_builder.read().build());

    let profile = use_memo(move || solution_builder.read().profile());

    let selected_fertilizers = use_memo(move || solution_builder.read().fertilizers());

    let mut profiles_listing = use_signal(move || {
        let profiles_list = profiles_storage.read().list();

        ProfilesListing::new(profiles_list)
    });

    let profiles = use_memo(move || profiles_listing.read().list());

    let mut fertilizers_listing = use_signal(move || {
        let fertilizers = fertilizers_storage.read().list();

        FertilizersListing::new(fertilizers)
    });

    let fertilizers = use_memo(move || fertilizers_listing.read().list());

    rsx! {
        section {
            class: "calculation-index__workspace",

            SolutionEditor {
                solution,
                fertilizers,
                selected_fertilizers,
                profile,
                profiles,
                on_profile_component_update: move |component| {
                    solution_builder.write().update_profile_component(component);
                },
                on_profile_change: move |profile_id: String| {
                    let profile = profiles_listing.read().find(profile_id);

                    solution_builder.write().update_profile(profile);
                },
                on_profile_search: move |search_query| {
                    profiles_listing.write().search(search_query);
                },
                on_fertilizer_select: move |fertilizer| {
                    solution_builder.write().add_fertilizer(fertilizer);
                },
                on_fertilizer_remove: move |fertilizer_id| {
                    solution_builder.write().remove_fertilizer(fertilizer_id);
                },
                on_fertilizer_search: move |search_query| {
                    fertilizers_listing.write().search(search_query);
                },
                on_water_amount_change: move |water_amount| {
                    println!("on_water_amount_change {}", water_amount);
                },
            }
        }

        section {
            class: "calculation-index__result",

            SolutionPreview {
                profile,
                solution,
                on_save: move |solution_name| {
                    let mut solution = solution.read().clone();

                    solution.set_name(solution_name);

                    solutions_storage.read().update(solution);

                    navigator().push(Route::SolutionsListingPage {});
                },
            }
        }
    }
}
