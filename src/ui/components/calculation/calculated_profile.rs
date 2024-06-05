use super::ResultProfileValue;
use crate::model::calculation::{Profile, ResultProfile};
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::ui::components::calculation::{NitrogenFormValue, NutrientRequirementInput};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CalculatedProfileProps {
    desired_profile: Signal<Profile>,
    result_profile: Signal<ResultProfile>,
}

#[component]
pub fn CalculatedProfile(props: CalculatedProfileProps) -> Element {
    let result_profile = props.result_profile.read();

    let desired_profile = props.desired_profile.read();

    rsx! {
        div {
            class: "calculated-profile",

            div {
                class: "calculated-profile__group",

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: result_profile[NutrientAmount::Nitrogen(0.0)],
                        desired_amount: desired_profile[NutrientAmount::Nitrogen(0.0)].value(),
                    }

                    NitrogenFormValue {
                        nitrogen_form: result_profile[NitrogenForm::Nitrate(0.0)],
                    }

                    NitrogenFormValue {
                        nitrogen_form: result_profile[NitrogenForm::Ammonium(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: result_profile[NutrientAmount::Phosphorus(0.0)],
                        desired_amount: desired_profile[NutrientAmount::Phosphorus(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: result_profile[NutrientAmount::Potassium(0.0)],
                        desired_amount: desired_profile[NutrientAmount::Potassium(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: result_profile[NutrientAmount::Calcium(0.0)],
                        desired_amount: desired_profile[NutrientAmount::Calcium(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: result_profile[NutrientAmount::Magnesium(0.0)],
                        desired_amount: desired_profile[NutrientAmount::Magnesium(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: result_profile[NutrientAmount::Sulfur(0.0)],
                        desired_amount: desired_profile[NutrientAmount::Sulfur(0.0)].value(),
                    }
                }
            }

            div {
                class: "calculated-profile__group",

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: result_profile[NutrientAmount::Iron(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: result_profile[NutrientAmount::Manganese(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: result_profile[NutrientAmount::Copper(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: result_profile[NutrientAmount::Zinc(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: result_profile[NutrientAmount::Boron(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: result_profile[NutrientAmount::Molybdenum(0.0)],
                    }
                }
            }
        }
    }
}
