use super::NitrogenFormValue;
use crate::model::calculation::Profile;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::ui::components::calculation::NutrientRequirementInput;
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

    rsx! {
        div {
            class: "desired-profile",

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
