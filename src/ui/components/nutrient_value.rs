use dioxus::prelude::*;

fn round(value: &String) -> String {
    format!("{:.3}", value.parse::<f64>().unwrap())
}

#[derive(Props, PartialEq, Clone)]
pub struct NutrientValueProps {
    symbol: String,
    value: f64,
    state: Option<String>,
    on_change: Option<EventHandler<f64>>,
}

#[component]
pub fn NutrientValue(props: NutrientValueProps) -> Element {
    let state = props.state.unwrap_or(String::from("default"));

    let mut value = use_signal(|| round(&props.value.to_string()));

    rsx! {
        div {
            class: "nutrient-value nutrient-value_state-{state}",

            span {
                class: "nutrient-value__symbol",
                {props.symbol},
            }

            match props.on_change {
                Some(on_change) => rsx! {
                    input {
                        class: "nutrient-value__value",
                        r#type: "text",
                        size: 1,
                        value,
                        oninput: move |event| {
                            let new_value = event.value().parse().unwrap_or(0.0);
                            value.set(event.value());
                            on_change.call(new_value);
                        },
                        onfocusout: move |_| {
                            let new_value = round(&value.read());
                            value.set(new_value);
                        },
                    }
                },

                None => rsx! {
                    input {
                        class: "nutrient-value__value",
                        r#type: "text",
                        disabled: true,
                        size: 1,
                        value,
                    }
                }
            }
        }
    }
}
