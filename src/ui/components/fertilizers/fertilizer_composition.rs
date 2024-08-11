use crate::ui::components::fertilizers::NutrientContentValue;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Text, Title};
use dioxus::prelude::*;
use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerCompositionProps {
    fertilizer: Memo<Fertilizer>,
}

#[component]
pub fn FertilizerComposition(props: FertilizerCompositionProps) -> Element {
    let fertilizer = &*props.fertilizer.read();

    rsx! {
        Column {
            gap: "medium",

            Row {
                Title {
                    size: "small",
                    "Питательные вещества",
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Макроэлементы",
                }

                Row {
                    gap: "small",

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Nitrogen),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Phosphorus),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Potassium),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Calcium),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Magnesium),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Sulfur),
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

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::NitrogenNitrate),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::NitrogenAmmonium),
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

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Iron),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Manganese),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Copper),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Zinc),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Boron),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients().value_of(Nutrient::Molybdenum),
                    }
                }
            }
        }
    }
}
