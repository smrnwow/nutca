use crate::model::calculation::Calculation;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::storage::{FertilizersStorage, SolutionsStorage};
use crate::ui::components::solutions::{SolutionEditor, SolutionPreview};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn update_list(mut list: Signal<Vec<(bool, Fertilizer)>>, item_id: String, selected: bool) {
    if let Some(item) = list.write().iter_mut().find(|item| item.1.id() == item_id) {
        item.0 = selected;
    }
}

fn calculate(fertilizers: Vec<Fertilizer>, profile: Profile) -> Solution {
    if fertilizers.len() > 0 {
        if let Ok(solution) = Calculation::new(profile, fertilizers.clone())
            .unwrap()
            .solve(1)
        {
            return solution;
        } else {
            return Solution::empty(fertilizers);
        }
    } else {
        return Solution::empty(fertilizers);
    }
}

#[component]
pub fn SolutionAddPage() -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut profile = use_signal(|| Profile::new());

    let mut solution = use_signal(|| Solution::from(profile.read().clone()));

    let fertilizers_list: Signal<Vec<(bool, Fertilizer)>> = use_signal(|| {
        let mut list: Vec<(bool, Fertilizer)> = vec![];

        for fertilizer in fertilizers_storage.read().list() {
            list.push((false, fertilizer));
        }

        list
    });

    let fertilizers_selected: Memo<Vec<Fertilizer>> = use_memo(move || {
        fertilizers_list
            .read()
            .iter()
            .filter(|(is_selected, _)| *is_selected)
            .map(|(_, fertilizer)| fertilizer.clone())
            .collect()
    });

    rsx! {
        section {
            class: "calculation-index__workspace",

            SolutionEditor {
                solution,
                fertilizers: fertilizers_list,
                profile,
                on_component_update: move |component| {
                    profile.write().set_component(component);

                    let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                    *solution.write() = result;
                },
                on_profile_change: move |new_profile: Option<Profile>| {
                    match new_profile {
                        Some(new_profile) => {
                            *profile.write() = new_profile;
                        },
                        None => {
                            *profile.write() = Profile::new();
                        }
                    }

                    let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                    *solution.write() = result;
                },
                on_fertilizer_select: move |(selected, fertilizer_id)| {
                    update_list(fertilizers_list, fertilizer_id, selected);

                    let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                    *solution.write() = result;
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
                    solution.write().set_name(solution_name);

                    solution.write().create_id();

                    println!("solution {:#?}", solution.read());

                    solutions_storage.read().add(solution.read().clone());

                    navigator().push(Route::SolutionsListingPage {});
                },
            }
        }
    }
}
