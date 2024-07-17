use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::ui::components::fertilizers::NutrientContentValue;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::utils::{Text, Title};
use dioxus::prelude::*;

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
                    ReferenceBadge {
                        article_id: "fertilizer-editor-nutrients",
                    },
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
                        nutrient: fertilizer.nutrients[Nutrient::Nitrogen],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Phosphorus],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Potassium],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Calcium],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Magnesium],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Sulfur],
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
                        nutrient: fertilizer.nutrients[Nutrient::NitrogenNitrate],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::NitrogenAmmonium],
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
                        nutrient: fertilizer.nutrients[Nutrient::Iron],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Manganese],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Copper],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Zinc],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Boron],
                    }

                    NutrientContentValue {
                        nutrient: fertilizer.nutrients[Nutrient::Molybdenum],
                    }
                }
            }
        }
    }
}
