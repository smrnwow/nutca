use crate::ui::components::fertilizers::NutrientContentValue;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Text, Title};
use dioxus::prelude::*;
use nutca::chemistry::Nutrient;
use nutca::fertilizers::Fertilizer;

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
                        nutrient: fertilizer.nutrient_amount(Nutrient::Nitrogen),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Phosphorus),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Potassium),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Calcium),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Magnesium),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Sulfur),
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
                        nutrient: fertilizer.nutrient_amount(Nutrient::NitrogenNitrate),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::NitrogenAmmonium),
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
                        nutrient: fertilizer.nutrient_amount(Nutrient::Iron),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Manganese),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Copper),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Zinc),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Boron),
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrient_amount(Nutrient::Molybdenum),
                    }
                }
            }
        }
    }
}
