use crate::ui::components::utils::icons::{Minus, Plus};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FloatFieldProps {
    maximum: Option<f64>,
    size: Option<String>,
    label: Option<String>,
    value: f64,
    units: Option<String>,
    on_change: EventHandler<f64>,
}

#[component]
pub fn FloatField(props: FloatFieldProps) -> Element {
    let size = props.size.unwrap_or(String::from("small"));

    let maximum = props.maximum.unwrap_or(1000000.0);

    rsx! {
        div {
            class: "number-field number-field_size-{size}",

            if let Some(label) = props.label {
                span {
                    class: "number-field__label",
                    "{label}",
                }
            }

            label {
                class: "number-field__value",

                button {
                    class: "number-field__button number-field__button_left",
                    onclick: move |_| {
                        if props.value >= 1.0 {
                            props.on_change.call(props.value - 1.0);
                        }
                    },
                    Minus {},
                }

                input {
                    class: "number-field__input",
                    r#type: "text",
                    size: 1,
                    value: "{props.value}",
                    oninput: move |event| {
                        let value = event.value().parse().unwrap_or(0.0);

                        if value < maximum {
                            props.on_change.call(value);
                        } else {
                            props.on_change.call(maximum);
                        }
                    },
                }

                button {
                    class: "number-field__button number-field__button_right",
                    onclick: move |_| {
                        if props.value < maximum {
                            props.on_change.call(props.value + 1.0);
                        }
                    },
                    Plus {},
                }
            }
        }
    }
}
