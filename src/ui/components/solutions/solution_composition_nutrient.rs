use crate::model::chemistry::Nutrient;
use crate::model::solutions::NutrientResult;
use crate::ui::components::utils::Tooltip;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

fn nutrient_class(nutrient_result: NutrientResult) -> String {
    let diff_percentage = nutrient_result.diff();

    if diff_percentage == 0. {
        return String::from("solution-composition-nutrient solution-composition-nutrient_green");
    }

    if diff_percentage <= 10. {
        return String::from("solution-composition-nutrient solution-composition-nutrient_yellow");
    }

    String::from("solution-composition-nutrient solution-composition-nutrient_red")
}

fn tooltip_text(nutrient_result: NutrientResult) -> String {
    let diff_percentage = nutrient_result.diff();

    if diff_percentage == 0. {
        return String::from("равно ожидаемому");
    }

    format!("превышает заданное значение на {}%", diff_percentage)
}

#[derive(Props, PartialEq, Clone)]
pub struct SolutionCompositionNutrientProps {
    nutrient: Nutrient,
    nutrient_result: NutrientResult,
}

#[component]
pub fn SolutionCompositionNutrient(props: SolutionCompositionNutrientProps) -> Element {
    rsx! {
        div {
            class: nutrient_class(props.nutrient_result),

            Tooltip {
                text: tooltip_text(props.nutrient_result),

                NutrientValue {
                    symbol: props.nutrient.symbol(),
                    value: props.nutrient.value(),
                }
            }
        }
    }
}
