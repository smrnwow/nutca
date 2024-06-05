use crate::model::calculation::{Calculation, Profile, ResultProfile};
use crate::model::fertilizers::Fertilizer;
use crate::storage::FertilizersStorage;
use crate::ui::components::calculation::{CalculatedProfile, DesiredProfile, FertilizersAmount};
use crate::ui::components::utils::icons::SearchIcon;
use dioxus::prelude::*;

fn update_list(mut list: Signal<Vec<(bool, Fertilizer)>>, item_id: String, value: String) {
    println!("event {} {}", item_id, value);

    if let Some(item) = list.write().iter_mut().find(|item| item.1.id() == item_id) {
        item.0 = value.parse().unwrap();
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
                    "Редактор профиля"
                }

                div {
                    class: "calculation-index__step",

                    span {
                        class: "calculation-index__step-number",
                        "1"
                    }

                    h6 {
                        class: "calculation-index__subtitle",
                        "Заполните желаемый профиль питания"
                    }
                }

                DesiredProfile {
                    profile,
                    on_requirement_update: move |nutrient_requirement| {
                        profile.write().set_nutrient(nutrient_requirement);

                        let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                        *result_profile.write() = result;
                    },
                    on_nitrogen_form_update: move |nitrogen_form| {
                        profile.write().set_nitrogen_form(nitrogen_form);

                        let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                        *result_profile.write() = result;
                    }
                }
            }

            div {
                class: "calculation-index__water",

                div {
                    class: "calculation-index__step",

                    span {
                        class: "calculation-index__step-number",
                        "2"
                    }

                    h6 {
                        class: "calculation-index__subtitle",
                        "Укажите объем и состав воды"
                    }
                }

                div {
                    class: "calculation-index__water-amount",

                    label {
                        class: "calculation-index__water-amount-label",

                        input {
                            class: "calculation-index__water-amount-input",
                            r#type: "number",
                            value: 1,
                        }

                        span {
                            class: "calculation-index__water-amount-tip",
                            "литр"
                        }
                    }

                }
            }

            div {
                class: "calculation-index__fertilizers",

                div {
                    class: "calculation-index__step",

                    span {
                        class: "calculation-index__step-number",
                        "3"
                    }

                    h6 {
                        class: "calculation-index__subtitle",
                        "Выберите удобрения"
                    }
                }

                div {
                    class: "fertilizers-browser",

                    div {
                        class: "fertilizers-browser__search",

                        label {
                            class: "fertilizers-browser__search-label",

                            SearchIcon {}

                            input {
                                class: "fertilizers-browser__search-input",
                                r#type: "text",
                                placeholder: "поиск удобрения",
                            }
                        }
                    }

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

                                            let result = calculate(fertilizers_selected.read().clone(), profile.read().clone());

                                            *result_profile.write() = result;
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

                                        for nutrient in fertilizer.nutrients() {
                                            div {
                                                class: "fertilizers-browser__nutrient",

                                                span {
                                                    class: "fertilizers-browser__nutrient-symbol",
                                                    "{nutrient.symbol()}",
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
        }

        div {
            class: "calculation-index__result",

            h3 {
                class: "calculation-index__title",
                "Результат"
            }

            CalculatedProfile {
                desired_profile: profile,
                result_profile,
            }

            FertilizersAmount {
                result_profile,
            }
        }
    }
}
