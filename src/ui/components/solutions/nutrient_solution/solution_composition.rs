use super::SolutionCompositionNutrient;
use crate::model::chemistry::Nutrient;
use crate::model::solutions::{Diff, NutritionContent};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionCompositionProps {
    nutrition_content: Memo<NutritionContent>,
    diff: Memo<Diff>,
}

#[component]
pub fn SolutionComposition(props: SolutionCompositionProps) -> Element {
    let nutrition_content = props.nutrition_content.read();

    let diff = props.diff.read();

    let badge = true;

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

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Nitrogen),
                        nutrient_result: diff.nutrient_diff(Nutrient::Nitrogen),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Phosphorus),
                        nutrient_result: diff.nutrient_diff(Nutrient::Phosphorus),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Potassium),
                        nutrient_result: diff.nutrient_diff(Nutrient::Potassium),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Calcium),
                        nutrient_result: diff.nutrient_diff(Nutrient::Calcium),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Magnesium),
                        nutrient_result: diff.nutrient_diff(Nutrient::Magnesium),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Sulfur),
                        nutrient_result: diff.nutrient_diff(Nutrient::Sulfur),
                    }
                }
            }

            Column {
                gap: "x-small",

                Text {
                    size: "x-small",
                    "Формы азота"
                }

                Row {
                    gap: "small",

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::NitrogenNitrate),
                        nutrient_result: diff.nutrient_diff(Nutrient::NitrogenNitrate),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::NitrogenAmmonium),
                        nutrient_result: diff.nutrient_diff(Nutrient::NitrogenAmmonium),
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

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Iron),
                        nutrient_result: diff.nutrient_diff(Nutrient::Iron),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Manganese),
                        nutrient_result: diff.nutrient_diff(Nutrient::Manganese),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Copper),
                        nutrient_result: diff.nutrient_diff(Nutrient::Copper),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Zinc),
                        nutrient_result: diff.nutrient_diff(Nutrient::Zinc),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Boron),
                        nutrient_result: diff.nutrient_diff(Nutrient::Boron),
                    }

                    SolutionCompositionNutrient {
                        badge,
                        nutrient: nutrition_content.value_of(Nutrient::Molybdenum),
                        nutrient_result: diff.nutrient_diff(Nutrient::Molybdenum),
                    }
                }
            }
        }
    }
}
