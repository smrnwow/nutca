use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::SolutionBuilder;
use crate::storage::{FertilizersStorage, SolutionsStorage};
use crate::ui::components::solutions::{SolutionEditor, SolutionPreview};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionAddPage() -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut solution_builder = use_signal(|| SolutionBuilder::new());

    let fertilizers_list = fertilizers_storage.read().list();

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
                on_profile_change: move |new_profile: Option<Profile>| {
                    match new_profile {
                        Some(new_profile) => {
                            solution_builder.write().update_profile(new_profile);
                        },
                        None => {
                            solution_builder.write().update_profile(Profile::new());
                        }
                    }
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
