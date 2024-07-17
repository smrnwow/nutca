use crate::model::chemistry::{Nutrient, NutrientAmount};
use crate::model::profiles::Profile;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileNutrientInput;
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileFormProps {
    profile: Memo<Profile>,
    on_nutrient_update: EventHandler<NutrientAmount>,
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
                        nutrient: profile.nutrients[Nutrient::Nitrogen],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Phosphorus],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Potassium],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Calcium],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Magnesium],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Sulfur],
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
                        nutrient: profile.nutrients[Nutrient::NitrogenNitrate],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::NitrogenAmmonium],
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
                        nutrient: profile.nutrients[Nutrient::Iron],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Manganese],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Copper],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Zinc],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Boron],
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrients[Nutrient::Molybdenum],
                        on_update: props.on_nutrient_update,
                    }
                }
            }
        }
    }
}
