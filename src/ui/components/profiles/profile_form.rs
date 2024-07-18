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
                        nutrient: profile.nutrient_requirement(Nutrient::Nitrogen),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Phosphorus),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Potassium),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Calcium),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Magnesium),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Sulfur),
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
                        nutrient: profile.nutrient_requirement(Nutrient::NitrogenNitrate),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::NitrogenAmmonium),
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
                        nutrient: profile.nutrient_requirement(Nutrient::Iron),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Manganese),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Copper),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Zinc),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Boron),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: profile.nutrient_requirement(Nutrient::Molybdenum),
                        on_update: props.on_nutrient_update,
                    }
                }
            }
        }
    }
}
