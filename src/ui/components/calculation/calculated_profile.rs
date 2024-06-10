use super::ResultProfileValue;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::ui::components::calculation::{NitrogenFormValue, NutrientRequirementInput};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CalculatedProfileProps {
    profile: Signal<Profile>,
    solution: Signal<Solution>,
}

#[component]
pub fn CalculatedProfile(props: CalculatedProfileProps) -> Element {
    let profile = props.profile.read();

    let solution = props.solution.read();

    rsx! {
        div {
            class: "calculated-profile",

            div {
                class: "calculated-profile__group",

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: solution[NutrientAmount::Nitrogen(0.0)],
                        desired_amount: profile[NutrientAmount::Nitrogen(0.0)].value(),
                    }

                    NitrogenFormValue {
                        nitrogen_form: solution[NitrogenForm::Nitrate(0.0)],
                    }

                    NitrogenFormValue {
                        nitrogen_form: solution[NitrogenForm::Ammonium(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: solution[NutrientAmount::Phosphorus(0.0)],
                        desired_amount: profile[NutrientAmount::Phosphorus(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: solution[NutrientAmount::Potassium(0.0)],
                        desired_amount: profile[NutrientAmount::Potassium(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: solution[NutrientAmount::Calcium(0.0)],
                        desired_amount: profile[NutrientAmount::Calcium(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: solution[NutrientAmount::Magnesium(0.0)],
                        desired_amount: profile[NutrientAmount::Magnesium(0.0)].value(),
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    ResultProfileValue {
                        nutrient_amount: solution[NutrientAmount::Sulfur(0.0)],
                        desired_amount: profile[NutrientAmount::Sulfur(0.0)].value(),
                    }
                }
            }

            div {
                class: "calculated-profile__group",

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: solution[NutrientAmount::Iron(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: solution[NutrientAmount::Manganese(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: solution[NutrientAmount::Copper(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: solution[NutrientAmount::Zinc(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: solution[NutrientAmount::Boron(0.0)],
                    }
                }

                div {
                    class: "nutrient-value__nutrient",

                    NutrientRequirementInput {
                        nutrient_amount: solution[NutrientAmount::Molybdenum(0.0)],
                    }
                }
            }
        }
    }
}
