use crate::model::chemistry::Nutrient;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::solutions::SolutionCompositionNutrient;
use crate::ui::components::utils::Text;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionCompositionProps {
    solution: Memo<Solution>,
}

#[component]
pub fn SolutionComposition(props: SolutionCompositionProps) -> Element {
    let solution = props.solution.read();

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
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Nitrogen],
                        nutrient_result: solution.nutrient_result(Nutrient::Nitrogen),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Phosphorus],
                        nutrient_result: solution.nutrient_result(Nutrient::Phosphorus),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Potassium],
                        nutrient_result: solution.nutrient_result(Nutrient::Potassium),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Calcium],
                        nutrient_result: solution.nutrient_result(Nutrient::Calcium),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Magnesium],
                        nutrient_result: solution.nutrient_result(Nutrient::Magnesium),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Sulfur],
                        nutrient_result: solution.nutrient_result(Nutrient::Sulfur),
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
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::NitrogenNitrate],
                        nutrient_result: solution.nutrient_result(Nutrient::NitrogenNitrate),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::NitrogenAmmonium],
                        nutrient_result: solution.nutrient_result(Nutrient::NitrogenAmmonium),
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
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Iron],
                        nutrient_result: solution.nutrient_result(Nutrient::Iron),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Manganese],
                        nutrient_result: solution.nutrient_result(Nutrient::Manganese),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Copper],
                        nutrient_result: solution.nutrient_result(Nutrient::Copper),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Zinc],
                        nutrient_result: solution.nutrient_result(Nutrient::Zinc),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Boron],
                        nutrient_result: solution.nutrient_result(Nutrient::Boron),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution.nutrients[Nutrient::Molybdenum],
                        nutrient_result: solution.nutrient_result(Nutrient::Molybdenum),
                    }
                }
            }
        }
    }
}
