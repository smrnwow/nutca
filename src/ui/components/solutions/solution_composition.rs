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
            gap: "small",

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
                        nutrient: solution[Nutrient::Nitrogen(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Nitrogen(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Phosphorus(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Phosphorus(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Potassium(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Potassium(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Calcium(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Calcium(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Magnesium(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Magnesium(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Sulfur(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Sulfur(0.0)),
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
                        nutrient: solution[Nutrient::NitrogenNitrate(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::NitrogenNitrate(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::NitrogenAmmonium(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::NitrogenAmmonium(0.0)),
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
                        nutrient: solution[Nutrient::Iron(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Iron(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Manganese(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Manganese(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Copper(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Copper(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Zinc(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Zinc(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Boron(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Boron(0.0)),
                    }

                    SolutionCompositionNutrient {
                        badge: solution.is_empty(),
                        nutrient: solution[Nutrient::Molybdenum(0.0)],
                        nutrient_result: solution.nutrient_result(Nutrient::Molybdenum(0.0)),
                    }
                }
            }
        }
    }
}
