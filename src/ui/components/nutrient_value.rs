use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct NutrientValueProps {
    symbol: String,
    value: f64,
    on_change: Option<EventHandler<f64>>,
}

#[component]
pub fn NutrientValue(props: NutrientValueProps) -> Element {
    rsx! {
        div {
            class: "nutrient-value",

            span {
                class: "nutrient-value__symbol",
                "{props.symbol}"
            }

            if props.on_change.is_some() {
                input {
                    class: "nutrient-value__value",
                    r#type: "text",
                    size: 1,
                    value: "{round(props.value)}",
                    oninput: move |event| props.on_change.unwrap().call(event.value().parse().unwrap_or(0.0)),
                }
            } else {
                input {
                    class: "nutrient-value__value",
                    r#type: "text",
                    disabled: true,
                    size: 1,
                    value: "{round(props.value)}",
                }
            }
        }
    }
}
