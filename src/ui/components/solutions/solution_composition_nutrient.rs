use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::solutions::NutrientResult;
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

#[derive(Props, PartialEq, Clone)]
pub struct SolutionCompositionNutrientProps {
    nutrient_amount: Option<NutrientAmount>,
    nitrogen_form: Option<NitrogenForm>,
    nutrient_result: NutrientResult,
    on_update: Option<EventHandler<NutrientAmount>>,
}

#[component]
pub fn SolutionCompositionNutrient(props: SolutionCompositionNutrientProps) -> Element {
    rsx! {
        div {
            class: nutrient_class(props.nutrient_result),

            if let Some(nutrient_amount) = props.nutrient_amount {
                NutrientValue {
                    symbol: nutrient_amount.symbol(),
                    value: nutrient_amount.value(),
                }
            }

            if let Some(nitrogen_form) = props.nitrogen_form {
                NutrientValue {
                    symbol: nitrogen_form.symbol(),
                    value: nitrogen_form.value(),
                }
            }
        }
    }
}
