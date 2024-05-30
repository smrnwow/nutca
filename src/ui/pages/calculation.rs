use crate::model::calculation::{Calculation, Profile, ResultProfile};
use crate::model::fertilizers::Fertilizer;
use crate::storage::FertilizersStorage;
use crate::ui::components::calculation::{CalculatedProfile, Profile};
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

fn update_list(mut list: Signal<Vec<(bool, Fertilizer)>>, item_id: String, value: String) {
    println!("event {} {}", item_id, value);

    if let Some(item) = list.write().iter_mut().find(|item| item.1.id() == item_id) {
        item.0 = value.parse().unwrap();
    }
}

#[component]
pub fn Calculation() -> Element {
    let mut profile = use_signal(|| Profile::new());

    let mut result_profile = use_signal(|| ResultProfile::new());

    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

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
        div {
            class: "calculation-index__workspace",

            div {
                class: "calculation-index__profile",

                h3 {
                    class: "calculation-index__title",
                    "Профиль"
                }

                Profile {
                    profile,
                    on_requirement_update: move |nutrient_requirement| {
                        profile.write().set_nutrient(nutrient_requirement);

                        if let Ok(result) = Calculation::new(profile.read().clone(), fertilizers_selected.read().clone())
                        .unwrap()
                        .solve(1) {
                            *result_profile.write() = result;
                        } else {
                            *result_profile.write() = ResultProfile::empty(fertilizers_selected.read().clone());
                        }
                    },
                }
            }

            div {
                class: "calculation-index__fertilizers",

                h3 {
                    class: "calculation-index__title",
                    "Удобрения"
                }

                div {
                    class: "fertilizers-browser",

                    div {
                        class: "fertilizers-browser__list",

                        for (is_selected, fertilizer) in fertilizers_list.read().clone() {
                            div {
                                class: "fertilizers-browser__item",
                                key: "{fertilizer.id()}",

                                div {
                                    class: "fertilizers-browser__selector",

                                    input {
                                        class: "fertilizers-browser__button",
                                        r#type: "checkbox",
                                        value: "{is_selected}",
                                        onchange: move |event| {
                                            update_list(fertilizers_list, fertilizer.id(), event.value());

                                            if let Ok(result) = Calculation::new(profile.read().clone(), fertilizers_selected.read().clone())
                                                .unwrap()
                                                .solve(1) {
                                                    *result_profile.write() = result;
                                                } else {
                                                    *result_profile.write() = ResultProfile::empty(fertilizers_selected.read().clone());
                                                }
                                        },
                                    }
                                }

                                div {
                                    class: "fertilizers-browser__info",

                                    p {
                                        class: "fertilizers-browser__name",
                                        "{fertilizer.name()}"
                                    }

                                    div {
                                        class: "fertilizers-browser__nutrients",

                                        for nutrient in fertilizer.nutrient_contents().nutrients() {
                                            NutrientValue {
                                                symbol: nutrient.symbol(),
                                                value: nutrient.value(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        div {
            class: "calculation-index__result",

            h3 {
                class: "calculation-index__title",
                "Результат"
            }

            CalculatedProfile {
                result_profile
            }
        }
    }
}
