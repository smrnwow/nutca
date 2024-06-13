use crate::model::chemistry::Nutrient;
use crate::model::profiles::Profile;
use crate::ui::components::profiles::ProfileNutrientInput;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileFormProps {
    profile: Memo<Profile>,
    on_nutrient_update: EventHandler<Nutrient>,
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

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Nitrogen(0.0)],
                            on_update: props.on_nutrient_update,
                        }

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::NitrogenNitrate(0.0)],
                            on_update: props.on_nutrient_update,
                        }

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::NitrogenAmmonium(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Phosphorus(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Potassium(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Calcium(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Magnesium(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Sulfur(0.0)],
                            on_update: props.on_nutrient_update,
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

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Iron(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Manganese(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Copper(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Zinc(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Boron(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }

                    div {
                        class: "profile-form__union",

                        ProfileNutrientInput {
                            nutrient: profile[Nutrient::Molybdenum(0.0)],
                            on_update: props.on_nutrient_update,
                        }
                    }
                }
            }
        }
    }
}
