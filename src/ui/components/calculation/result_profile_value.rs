use crate::model::chemistry::NutrientAmount;
use crate::ui::components::NutrientValue;
use dioxus::prelude::*;

fn value_status(result_value: f64, desired_value: f64) -> String {
    let diff = result_value - desired_value;

    if diff < 0. || diff > 20. {
        return String::from("red");
    }

    if diff >= 0. && diff <= 5. {
        return String::from("green");
    }

    String::from("yellow")
}

#[derive(Props, PartialEq, Clone)]
pub struct ResultProfileValueProps {
    nutrient_amount: NutrientAmount,
    desired_amount: f64,
    on_update: Option<EventHandler<NutrientAmount>>,
}

#[component]
pub fn ResultProfileValue(props: ResultProfileValueProps) -> Element {
    println!(
        "{} is {} = {}, {}",
        props.nutrient_amount.symbol(),
        value_status(props.nutrient_amount.value(), props.desired_amount),
        props.nutrient_amount.value(),
        props.desired_amount,
    );

    rsx! {
        if let Some(on_update) = props.on_update {
            NutrientValue {
                symbol: props.nutrient_amount.symbol(),
                value: props.nutrient_amount.value(),
                on_change: move |value| on_update.call(props.nutrient_amount.new(value)),
            }
        } else {
            NutrientValue {
                symbol: props.nutrient_amount.symbol(),
                value: props.nutrient_amount.value(),
            }
        }
    }
}
