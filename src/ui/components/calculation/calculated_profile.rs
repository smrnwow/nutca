use crate::model::calculation::ResultProfile;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::ui::components::calculation::{NitrogenFormValue, NutrientRequirementInput};
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
                                nutrient_amount: result_profile[NutrientAmount::Nitrogen(0.0)],
                            }
                        }

                        div {
                            class: "nutrient-value__forms",

                            div {
                                class: "nutrient-value__form",

                                NitrogenFormValue {
                                    nitrogen_form: result_profile[NitrogenForm::Nitrate(0.0)],
                                    on_update: move |_| {},
                                }
                            }

                            div {
                                class: "nutrient-value__form",

                                NitrogenFormValue {
                                    nitrogen_form: result_profile[NitrogenForm::Ammonium(0.0)],
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
                                nutrient_amount: result_profile[NutrientAmount::Phosphorus(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Potassium(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Calcium(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Magnesium(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Sulfur(0.0)],
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
                                nutrient_amount: result_profile[NutrientAmount::Iron(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Manganese(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Copper(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Zinc(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Boron(0.0)],
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__nutrient",

                        div {
                            class: "nutrient-value__elemental",

                            NutrientRequirementInput {
                                nutrient_amount: result_profile[NutrientAmount::Molybdenum(0.0)],
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
