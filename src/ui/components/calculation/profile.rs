use crate::model::calculation::{NutrientRequirement, Profile};
use crate::ui::components::calculation::NutrientRequirementInput;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileProps {
    profile: Signal<Profile>,
    on_requirement_update: EventHandler<NutrientRequirement>,
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
                                nutrient_requirement: profile.nitrogen(),
                                on_update: props.on_requirement_update,
                            }
                        }

                        div {
                            class: "nutrient-value__forms",

                            div {
                                class: "nutrient-value__form",

                                NutrientRequirementInput {
                                    nutrient_requirement: profile.nitrogen_nitrate(),
                                    on_update: props.on_requirement_update,
                                }
                            }

                            div {
                                class: "nutrient-value__form",

                                NutrientRequirementInput {
                                    nutrient_requirement: profile.nitrogen_ammonium(),
                                    on_update: props.on_requirement_update,
                                }
                            }
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.phosphor(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.potassium(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.calcium(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.magnesium(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.sulfur(),
                            on_update: props.on_requirement_update,
                        }
                    }
                }

                div {
                    class: "profile__group",

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.iron(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.manganese(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.copper(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.zinc(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.boron(),
                            on_update: props.on_requirement_update,
                        }
                    }

                    div {
                        class: "nutrient-value__elemental",

                        NutrientRequirementInput {
                            nutrient_requirement: profile.molybdenum(),
                            on_update: props.on_requirement_update,
                        }
                    }
                }
            }
    }
}
