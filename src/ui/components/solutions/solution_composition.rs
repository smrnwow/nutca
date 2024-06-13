use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::solutions::Solution;
use crate::ui::components::solutions::SolutionCompositionNutrient;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionCompositionProps {
    solution: Memo<Solution>,
}

#[component]
pub fn SolutionComposition(props: SolutionCompositionProps) -> Element {
    let solution = props.solution.read();

    rsx! {
        div {
            class: "calculated-profile",

            div {
                class: "calculated-profile__group",

                div {
                    class: "solution-composition-union",

                    SolutionCompositionNutrient {
                        nutrient_amount: solution[NutrientAmount::Nitrogen(0.0)],
                        nutrient_result: solution.nutrient_amount_result(NutrientAmount::Nitrogen(0.0)),
                    }

                    SolutionCompositionNutrient {
                        nitrogen_form: solution[NitrogenForm::Nitrate(0.0)],
                        nutrient_result: solution.nitrogen_form_result(NitrogenForm::Nitrate(0.0)),
                    }

                    SolutionCompositionNutrient {
                        nitrogen_form: solution[NitrogenForm::Ammonium(0.0)],
                        nutrient_result: solution.nitrogen_form_result(NitrogenForm::Ammonium(0.0)),
                    }
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Phosphorus(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Phosphorus(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Potassium(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Potassium(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Calcium(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Calcium(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Magnesium(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Magnesium(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Sulfur(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Sulfur(0.0)),
                }
            }

            div {
                class: "calculated-profile__group",

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Iron(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Iron(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Manganese(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Manganese(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Copper(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Copper(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Zinc(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Zinc(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Boron(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Boron(0.0)),
                }

                SolutionCompositionNutrient {
                    nutrient_amount: solution[NutrientAmount::Molybdenum(0.0)],
                    nutrient_result: solution.nutrient_amount_result(NutrientAmount::Molybdenum(0.0)),
                }
            }
        }
    }
}
