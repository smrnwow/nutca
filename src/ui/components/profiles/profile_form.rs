use crate::model::chemistry::Nutrient;
use crate::model::profiles::Profile;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileNutrientInput;
use crate::ui::components::utils::Text;
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
        Column {
            gap: "medium",

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Макроэлементы",
                }

                Row {
                    gap: "small",

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Nitrogen(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Phosphorus(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Potassium(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Calcium(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Magnesium(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Sulfur(0.0)],
                        on_update: props.on_nutrient_update,
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Формы азота",
                }

                Row {
                    gap: "small",

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::NitrogenNitrate(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::NitrogenAmmonium(0.0)],
                        on_update: props.on_nutrient_update,
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Микроэлементы",
                }

                Row {
                    gap: "small",

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Iron(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Manganese(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Copper(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Zinc(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Boron(0.0)],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile[Nutrient::Molybdenum(0.0)],
                        on_update: props.on_nutrient_update,
                    }
                }
            }
        }
    }
}
