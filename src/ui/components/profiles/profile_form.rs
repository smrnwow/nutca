use super::ProfileComponentInput;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::profiles::{Component, Profile};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileFormProps {
    profile: Memo<Profile>,
    on_component_update: EventHandler<Component>,
}

#[component]
pub fn ProfileForm(props: ProfileFormProps) -> Element {
    let profile = props.profile.read();

    rsx! {
        div {
            class: "profile-form",

            div {
                class: "profile-form__macro",

                p {
                    class: "profile-form__text",
                    "Макроэлементы"
                }

                div {
                    class: "profile-form__group",

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Nitrogen(0.0)]),
                            on_update: props.on_component_update,
                        }

                        ProfileComponentInput {
                            component: Component::NitrogenForm(profile[NitrogenForm::Nitrate(0.0)]),
                            on_update: props.on_component_update,
                        }

                        ProfileComponentInput {
                            component: Component::NitrogenForm(profile[NitrogenForm::Ammonium(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Phosphorus(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Potassium(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Calcium(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Magnesium(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Sulfur(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }
                }
            }

            div {
                class: "profile-form__micro",

                p {
                    class: "profile-form__text",
                    "Микроэлементы"
                }

                div {
                    class: "profile-form__group",

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Iron(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Manganese(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Copper(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Zinc(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Boron(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileComponentInput {
                            component: Component::Nutrient(profile[NutrientAmount::Molybdenum(0.0)]),
                            on_update: props.on_component_update,
                        }
                    }
                }
            }
        }
    }
}
