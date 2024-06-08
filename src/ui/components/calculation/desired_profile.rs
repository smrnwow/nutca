use super::NitrogenFormValue;
use crate::model::calculation::Profile;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::ui::components::calculation::NutrientRequirementInput;
use crate::ui::components::utils::Dropdown;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DesiredProfileProps {
    profile: Signal<Profile>,
    on_requirement_update: EventHandler<NutrientAmount>,
    on_nitrogen_form_update: EventHandler<NitrogenForm>,
}

#[component]
pub fn DesiredProfile(props: DesiredProfileProps) -> Element {
    let profile = *props.profile.read();

    let profiles = use_signal(|| {
        vec![
            (String::from("grow"), String::from("grow")),
            (String::from("bloom"), String::from("bloom")),
        ]
    });

    let mut selected_profile = use_signal(|| (String::new(), String::new()));

    rsx! {
        div {
            class: "desired-profile",

            div {
                class: "desired_profile_-browser",

                Dropdown {
                    placeholder: "выбрать готовый профиль...",
                    value: selected_profile,
                    options: profiles.read().clone(),
                    on_search: move |search_query| {
                        println!("on_search {}", search_query);
                    },
                    on_select: move |value| {
                        let profiles = profiles.read().clone();

                        let profile = profiles.iter().find(|profile| profile.0 == value);

                        match profile {
                            Some(profile) => {
                                println!("on_select {:#?}", profile);
                                *selected_profile.write() = profile.clone();
                            },

                            None => {}
                        }
                    },
                    on_cancel: move |_| {
                        *selected_profile.write() = (String::new(), String::new());
                    },
                }
            }

            div {
                class: "desired-profile__macro",

                p {
                    class: "desired-profile__text",
                    "Макроэлементы"
                }

                div {
                    class: "desired-profile__group",

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Nitrogen(0.0)],
                            on_update: props.on_requirement_update,
                        }

                        NitrogenFormValue {
                            nitrogen_form: profile[NitrogenForm::Nitrate(0.0)],
                            on_update: props.on_nitrogen_form_update,
                        }

                        NitrogenFormValue {
                            nitrogen_form: profile[NitrogenForm::Ammonium(0.0)],
                            on_update: props.on_nitrogen_form_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Phosphorus(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Potassium(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Calcium(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Magnesium(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Sulfur(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }
                }
            }

            div {
                class: "desired-profile__micro",

                p {
                    class: "desired-profile__text",
                    "Микроэлементы"
                }

                div {
                    class: "desired-profile__group",

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Iron(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Manganese(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Copper(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Zinc(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Boron(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Molybdenum(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }
                }
            }
        }
    }
}
