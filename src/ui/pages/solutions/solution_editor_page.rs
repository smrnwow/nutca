use crate::model::calculation::{Calculation, Profile, ResultProfile};
use crate::model::fertilizers::Fertilizer;
use crate::storage::FertilizersStorage;
use crate::ui::components::calculation::{ResultSolutionPreview, SolutionEditorWorkspace};
use dioxus::prelude::*;

fn update_list(mut list: Signal<Vec<(bool, Fertilizer)>>, item_id: String, selected: bool) {
    if let Some(item) = list.write().iter_mut().find(|item| item.1.id() == item_id) {
        item.0 = selected;
    }
}

fn calculate(fertilizers: Vec<Fertilizer>, profile: Profile) -> ResultProfile {
    if fertilizers.len() > 0 {
        if let Ok(result) = Calculation::new(profile, fertilizers.clone())
            .unwrap()
            .solve(1)
        {
            return result;
        } else {
            return ResultProfile::empty(fertilizers);
        }
    } else {
        return ResultProfile::empty(fertilizers);
    }
}

#[component]
pub fn SolutionEditorPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut profile = use_signal(|| Profile::new());

    let mut result_profile = use_signal(|| ResultProfile::new());

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

            SolutionEditorWorkspace {
                fertilizers: fertilizers_list,
                profile,
                on_requirement_update: move |nutrient_amount| {
                    profile.write().set_nutrient(nutrient_amount);

                    let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                    *result_profile.write() = result;
                },
                on_nitrogen_form_update: move |nitrogen_form| {
                    profile.write().set_nitrogen_form(nitrogen_form);

                    let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                    *result_profile.write() = result;
                },
                on_fertilizer_select: move |(selected, fertilizer_id)| {
                    update_list(fertilizers_list, fertilizer_id, selected);

                    let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                    *result_profile.write() = result;
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

            ResultSolutionPreview {
                profile,
                result: result_profile,
                on_save: move |solution_name| {
                    println!("on_solution_save {}", solution_name);
                },
            }
        }
    }
}
