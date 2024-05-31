use super::NitrogenFormValue;
use crate::model::calculation::Profile;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::ui::components::calculation::NutrientRequirementInput;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileProps {
    profile: Signal<Profile>,
    on_requirement_update: EventHandler<NutrientAmount>,
    on_nitrogen_form_update: EventHandler<NitrogenForm>,
}

#[component]
pub fn Profile(props: ProfileProps) -> Element {
    let profile = *props.profile.read();

    rsx! {
            div {
                class: "profile",

                div {
                    class: "profile__group",

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: profile[NutrientAmount::Nitrogen(0.0)],
                                on_update: props.on_requirement_update,
                            }
                        }

                        div {
                            class: "nutrient-value__forms",

                            div {
                                class: "nutrient-value__form",

                                NitrogenFormValue {
                                    nitrogen_form: profile[NitrogenForm::Nitrate(0.0)],
                                    on_update: props.on_nitrogen_form_update,
                                }
                            }

                            div {
                                class: "nutrient-value__form",

                                NitrogenFormValue {
                                    nitrogen_form: profile[NitrogenForm::Ammonium(0.0)],
                                    on_update: props.on_nitrogen_form_update,
                                }
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Phosphorus(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Potassium(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Calcium(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Magnesium(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Sulfur(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }
                }

                div {
                    class: "profile__group",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Iron(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Manganese(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Copper(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Zinc(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Boron(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_amount: profile[NutrientAmount::Molybdenum(0.0)],
                            on_update: props.on_requirement_update,
                        }
                    }
                }
            }
    }
}
