use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::SolutionBuilder;
use crate::storage::{FertilizersStorage, ProfilesStorage, SolutionsStorage};
use crate::ui::components::solutions::{SolutionEditor, SolutionPreview};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionAddPage(profile_id: String) -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let fertilizers_list = fertilizers_storage.read().list();

    let mut solution_builder = use_signal(|| {
        let profile = profiles_storage.read().get(profile_id);

        match profile {
            Some(profile) => SolutionBuilder::base_on(profile),
            None => SolutionBuilder::new(),
        }
    });

    let profile = use_memo(move || solution_builder.read().profile());

    let solution = use_memo(move || solution_builder.read().build());

    let fertilizers: Memo<Vec<(bool, Fertilizer)>> = use_memo(move || {
        let mut list: Vec<(bool, Fertilizer)> = vec![];

        for fertilizer in fertilizers_list.clone() {
            list.push((
                solution_builder.read().contains_fertilizer(fertilizer.id()),
                fertilizer,
            ));
        }

        list
    });

    rsx! {
        section {
            class: "calculation-index__workspace",

            SolutionEditor {
                solution,
                fertilizers,
                profile,
                on_component_update: move |component| {
                    solution_builder.write().update_profile_component(component);
                },
                on_profile_change: move |profile: Option<Profile>| {
                    solution_builder.write().update_profile(profile);
                },
                on_fertilizer_add: move |fertilizer| {
                    solution_builder.write().add_fertilizer(fertilizer);
                },
                on_fertilizer_remove: move |fertilizer_id| {
                    solution_builder.write().remove_fertilizer(fertilizer_id);
                },
                on_fertilizer_search: move |search_query| {
                    println!("on_fertilizer_search {}", search_query);
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

                    solutions_storage.read().add(solution);

                    navigator().push(Route::SolutionsListingPage {});
                },
            }
        }
    }
}
