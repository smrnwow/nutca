use crate::model::calculation::ResultProfile;
use crate::ui::components::calculation::NutrientRequirementInput;
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct CalculatedProfileProps {
    result_profile: Signal<ResultProfile>,
}

#[component]
pub fn CalculatedProfile(props: CalculatedProfileProps) -> Element {
    let result_profile = props.result_profile.read();

    rsx! {
            div {
                class: "calculated-profile",

                div {
                    class: "calculated-profile__group",

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.nitrogen(),
                                on_update: move |_| {},
                            }
                        }

                        div {
                            class: "nutrient-value__forms",

                            div {
                                class: "nutrient-value__form",

                                NutrientRequirementInput {
                                    nutrient_requirement: result_profile.profile.nitrogen_nitrate(),
                                    on_update: move |_| {},
                                }
                            }

                            div {
                                class: "nutrient-value__form",

                                NutrientRequirementInput {
                                    nutrient_requirement: result_profile.profile.nitrogen_ammonium(),
                                    on_update: move |_| {},
                                }
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.phosphor(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.potassium(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.calcium(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.magnesium(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.sulfur(),
                                on_update: move |_| {},
                            }
                        }
                    }
                }

                div {
                    class: "calculated-profile__group",

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.iron(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.manganese(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.copper(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.zinc(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.boron(),
                                on_update: move |_| {},
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_requirement: result_profile.profile.molybdenum(),
                                on_update: move |_| {},
                            }
                        }
                    }
                }

                div {
                    class: "calculated-profile__fertilizers",

                    for fertilizer_weight in result_profile.fertilizers_weights.clone() {
                        div {
                            class: "calculated-profile__fertilizer-weight",

                            p {
                                class: "calculated-profile__fertilizer",
                                "{fertilizer_weight.fertilizer.name()}",
                            }

                            p {
                                class: "calculated-profile__weight",
                                "{round(fertilizer_weight.weight)} г",
                            }
                        }
                    }
                }
            }
    }
}
