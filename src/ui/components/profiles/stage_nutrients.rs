use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileNutrientInput;
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StageNutrientsProps {
    nutrients: Memo<Nutrients>,
    on_nutrient_update: EventHandler<NutrientAmount>,
}

#[component]
pub fn StageNutrients(props: StageNutrientsProps) -> Element {
    let nutrients = props.nutrients.read();

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
                        nutrient: nutrients.value_of(Nutrient::Nitrogen),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Phosphorus),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Potassium),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Calcium),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Magnesium),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Sulfur),
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
                        nutrient: nutrients.value_of(Nutrient::NitrogenNitrate),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::NitrogenAmmonium),
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
                        nutrient: nutrients.value_of(Nutrient::Iron),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Manganese),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Copper),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Zinc),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Boron),
                        on_update: props.on_nutrient_update,
                    }

                    ProfileNutrientInput {
                        nutrient: nutrients.value_of(Nutrient::Molybdenum),
                        on_update: props.on_nutrient_update,
                    }
                }
            }
        }
    }
}
